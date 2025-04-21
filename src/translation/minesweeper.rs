use super::Language;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MINESWEEPER_TRANSLATIONS: HashMap<&'static str, HashMap<Language, String>> = {
        let mut m = HashMap::new();

        let mut minesweeper_title = HashMap::new();
        minesweeper_title.insert(Language::English, "Minesweeper".to_string());
        minesweeper_title.insert(Language::Chinese, "扫雷".to_string());
        m.insert("minesweeper_title", minesweeper_title);

        // let mut welcome_to = HashMap::new();
        
    }
}