use dotenv_parser::parse_dotenv;

fn main() {
    let source = r#"
        ENV_FOR_HYDRO='testing 2' # another one here
        export USER_ID=5gpPN5rcv5G41U_S
        API_TOKEN=30af563ccc668bc8ced9e24e  # relax! these values are fake
        APP_SITE_URL=https://my.example.com
    "#;
    println!("{:#?}", parse_dotenv(source).unwrap());
}
