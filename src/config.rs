extern crate config;

use self::config::*;
use may::sync::RwLock;

pub const HASH_LENGTH: usize = 44;
pub const PUBKEY_LENGTH: usize = 44;
pub const SIG_LENGTH: usize = 88;
pub const MAX_COMPLEXITY: usize = 100;
pub const COUNT_WITNESSES: usize = 12;
pub const TOTAL_WHITEBYTES: i64 = 500_000_000_000_000;
pub const MAX_WITNESS_LIST_MUTATIONS: usize = 1;
pub const MAJORITY_OF_WITNESSES: usize = 7;
pub const VERSION: &str = "1.0";
pub const ALT: &str = "1";
pub const LIBRARY: &str = "rust-trustnote";
pub const LIBRARY_VERSION: &str = "0.1.0";
pub const PROGRAM: &str = "rust-trustnote-hub";
pub const PROGRAM_VESION: &str = "0.1.0";
pub const STALLED_TIMEOUT: usize = 10;
pub const MAX_MESSAGES_PER_UNIT: usize = 128;
pub const MAX_PARENT_PER_UNIT: usize = 16;
pub const MAX_AUTHORS_PER_UNIT: usize = 16;
pub const MAX_SPEND_PROOFS_PER_MESSAGE: usize = 128;
pub const MAX_INPUTS_PER_PAYMENT_MESSAGE: usize = 128;
pub const MAX_OUTPUTS_PER_PAYMENT_MESSAGE: usize = 128;
pub const MAX_AUTHENTIFIER_LENGTH: usize = 4096;
pub const COUNT_MC_BALLS_FOR_PAID_WITNESSING: u32 = 100;
pub const MAX_DATA_FEED_NAME_LENGTH: usize = 64;
pub const MAX_DATA_FEED_VALUE_LENGTH: usize = 64;
pub const MAX_ITEMS_IN_CACHE: usize = 1000;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings
            .merge(File::with_name("settings.json"))
            .expect("failed to load config");
        settings
    });
}

pub fn show_config() {
    println!("\nconfig:");
    println!("\tremote_hub = {}", get_remote_hub_url());
    println!("\thub_server_port = {}", get_hub_server_port());
    println!("\tremote_hub = {}", get_remote_hub_url());
    println!("\tdatabase_path = {:?}", get_database_path());
    println!("\n");
}

pub fn get_witnesses() -> [String; 12] {
    let cfg = CONFIG.read().unwrap();
    cfg.get::<[String; 12]>("witnesses")
        .expect("failed to read witnesses")
}

pub fn get_genesis_unit() -> String {
    let cfg = CONFIG.read().unwrap();
    cfg.get::<String>("genesis_unit")
        .expect("failed to read genesis unit")
}

pub fn get_remote_hub_url() -> String {
    let cfg = CONFIG.read().unwrap();
    cfg.get::<String>("remote_hub")
        .unwrap_or_else(|_| "127.0.0.1:6655".to_owned())
}

pub fn get_hub_server_port() -> u16 {
    let cfg = CONFIG.read().unwrap();
    cfg.get::<u16>("hub_server_port").unwrap_or(6615)
}

pub fn get_initial_db_path() -> String {
    let cfg = CONFIG.read().unwrap();
    cfg.get::<String>("initial_db_path")
        .unwrap_or_else(|_| "db/initial.trustnote.sqlite".to_owned())
}

pub fn get_database_path() -> ::std::path::PathBuf {
    use app_dirs::*;
    use std::fs;

    const APP_INFO: AppInfo = AppInfo {
        name: "rust-trustnote",
        author: "trustnote-hub",
    };

    let mut db_path = get_app_root(AppDataType::UserData, &APP_INFO)
        .unwrap_or_else(|e| panic!("failed to get app dir, err={}", e));
    if !db_path.exists() {
        fs::create_dir_all(&db_path)
            .unwrap_or_else(|e| panic!("failed to create database dir: {:?}, err={}", db_path, e));
    }
    db_path.push("trustnote.sqlite");
    db_path
}
