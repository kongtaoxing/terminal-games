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

        let mut game_win = HashMap::new();
        game_win.insert(Language::English, "Congratulations! You Win!".to_string());
        game_win.insert(Language::Chinese, "恭喜你！胜利了！".to_string());
        m.insert("game_win", game_win);

        let mut game_over = HashMap::new();
        game_over.insert(Language::English, "Game Over!".to_string());
        game_over.insert(Language::Chinese, "游戏结束！".to_string());
        m.insert("game_over", game_over);
        
        m
    };
}