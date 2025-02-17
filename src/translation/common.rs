use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

lazy_static! {
    pub static ref COMMON_TRANSLATIONS: HashMap<&'static str, HashMap<Language, String>> = {
        let mut m = HashMap::new();
        
        // Main Menu translations
        let mut menu_title = HashMap::new();
        menu_title.insert(Language::English, "Terminal Game Collection".to_string());
        menu_title.insert(Language::Chinese, "终端游戏集合".to_string());
        m.insert("menu_title", menu_title);

        let mut available_games = HashMap::new();
        available_games.insert(Language::English, "Available Games:".to_string());
        available_games.insert(Language::Chinese, "可用游戏：".to_string());
        m.insert("available_games", available_games);

        let mut controls = HashMap::new();
        controls.insert(Language::English, "Controls:".to_string());
        controls.insert(Language::Chinese, "控制：".to_string());
        m.insert("controls", controls);

        let mut controls_desc = HashMap::new();
        controls_desc.insert(Language::English, "- Use UP/DOWN arrows to select\n- Press ENTER to start game\n- T: Change language\n- Q: Quit game".to_string());
        controls_desc.insert(Language::Chinese, "- 使用上下方向键选择\n- 按回车键开始游戏\n- T：切换语言\n- Q：退出游戏".to_string());
        m.insert("controls_desc", controls_desc);

        let mut compiling = HashMap::new();
        compiling.insert(Language::English, "- Press p or Esc to pause game and pretend to compile code (fish).".to_string());
        compiling.insert(Language::Chinese, "- 游戏中按p或者Esc可暂停游戏并假装编译代码（摸鱼）".to_string());
        m.insert("compiling", compiling);

        m
    };
} 