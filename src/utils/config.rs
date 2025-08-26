use dotenvy::dotenv;

pub fn load() {
    dotenv().expect("DotEnv Err??");
}
