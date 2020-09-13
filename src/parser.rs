use std::collections::BTreeMap;

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "dotenv.pest"]
struct DotenvLineParser;

/// Parse the .env file source.
pub fn parse_dotenv(
    source: &str,
) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let mut map = BTreeMap::new();
    let pairs = DotenvLineParser::parse(Rule::env, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::kv => {
                if let Some((key, value)) = parse_kv(pair) {
                    map.insert(key, value);
                }
            }
            _ => {}
        }
    }
    Ok(map)
}

/// Parse a key-value pair.
fn parse_kv(pair: Pair<Rule>) -> Option<(String, String)> {
    match pair.as_rule() {
        Rule::kv => {
            let mut inner_rules = pair.into_inner(); // key ~ "=" ~ value
            let name: &str = inner_rules.next().unwrap().as_str();
            parse_value(inner_rules.next().unwrap()).map(|v| (name.into(), v))
        }
        _ => None,
    }
}

/// Parse a value, which might be a string or a naked variable.
fn parse_value(pair: Pair<Rule>) -> Option<String> {
    match pair.as_rule() {
        Rule::value => {
            let inner = pair.clone().into_inner().next();
            // If there are no inner pairs, the current value is a naked
            // variable, otherwise it's a string and we need to extract the
            // inner_sq or inner_dq pair.
            match inner {
                None => Some(pair.as_str().into()),
                Some(inner_pair) => match inner_pair.into_inner().next() {
                    None => None,
                    Some(inner_string) => Some(inner_string.as_str().into()),
                },
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_dotenv;
    use std::collections::BTreeMap;

    #[test]
    fn empty_file() {
        assert_eq!(parse_dotenv("").unwrap(), BTreeMap::new());
    }

    #[test]
    fn one_kv() {
        let bm = vec![("key", "value")]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
        assert_eq!(parse_dotenv("key = value").unwrap(), bm);
    }

    #[test]
    fn one_line() {
        let bm = vec![("key", "value")]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
        assert_eq!(parse_dotenv("key = value\n").unwrap(), bm);
    }

    #[test]
    fn two_lines() {
        let bm = vec![("key", "value"), ("key2", "value2")]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
        assert_eq!(parse_dotenv("key = value\nkey2 = value2").unwrap(), bm);
    }

    #[test]
    fn non_alphanumeric_chars() {
        let bm = vec![("key", "https://1.3.2.3:234/a?b=c")]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
        assert_eq!(parse_dotenv("key=https://1.3.2.3:234/a?b=c\n").unwrap(), bm);
    }

    #[test]
    fn export() {
        let bm = vec![("key", "value"), ("key2", "value2")]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
        assert_eq!(
            parse_dotenv("key = value\nexport key2 = value2").unwrap(),
            bm
        );
    }

    #[test]
    fn string_single_quotes() {
        let bm = vec![("key", "value"), ("key2", "val ue2")]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
        assert_eq!(parse_dotenv("key = value\nkey2 = 'val ue2'").unwrap(), bm);
    }

    #[test]
    fn string_double_quotes() {
        let bm = vec![("key", "value"), ("key2", "val ue2")]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
        assert_eq!(
            parse_dotenv("key = value\nkey2 = \"val ue2\"").unwrap(),
            bm
        );
    }

    #[test]
    fn comments() {
        let source = r#"
            # one here
            ENV_FOR_HYDRO=production # another one here
        "#;
        let bm = vec![("ENV_FOR_HYDRO", "production")]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect();
        assert_eq!(parse_dotenv(source).unwrap(), bm);
    }

    #[test]
    fn complete_dotenv() {
        let source = r#"
            ENV_FOR_HYDRO='testing 2' # another one here
            USER_ID=5gpPN5rcv5G41U_S
            API_TOKEN=30af563ccc668bc8ced9e24e  # relax! these values are fake
            APP_SITE_URL=https://my.example.com
        "#;
        let bm = vec![
            ("ENV_FOR_HYDRO", "testing 2"),
            ("USER_ID", "5gpPN5rcv5G41U_S"),
            ("API_TOKEN", "30af563ccc668bc8ced9e24e"),
            ("APP_SITE_URL", "https://my.example.com"),
        ]
        .into_iter()
        .map(|(a, b)| (a.into(), b.into()))
        .collect();
        assert_eq!(parse_dotenv(source).unwrap(), bm);
    }
}
