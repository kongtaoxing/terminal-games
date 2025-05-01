use super::Language;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MINESWEEPER_TRANSLATIONS: HashMap<&'static str, HashMap<Language, String>> = {
        let mut m = HashMap::new();

        let mut minesweeper_title = HashMap::new();
        minesweeper_title.insert(Language::English, "Minesweeper".to_string());
        minesweeper_title.insert(Language::Chinese, "扫雷".to_string());
        m.insert("minesweeper.minesweeper_title", minesweeper_title);

        let mut game_win = HashMap::new();
        game_win.insert(Language::English, "Congratulations! You Win!".to_string());
        game_win.insert(Language::Chinese, "恭喜你！胜利了！".to_string());
        m.insert("minesweeper.game_win", game_win);

        let mut game_over = HashMap::new();
        game_over.insert(Language::English, "Game Over!".to_string());
        game_over.insert(Language::Chinese, "游戏结束！".to_string());
        m.insert("minesweeper.game_over", game_over);

        let mut press_r_restart = HashMap::new();
        press_r_restart.insert(Language::English, "Press 'r' to restart".to_string());
        press_r_restart.insert(Language::Chinese, "按'r'键重新开始".to_string());
        m.insert("minesweeper.press_r_restart", press_r_restart);

        let mut how_to_play = HashMap::new();
        how_to_play.insert(Language::English, "How to Play:".to_string());
        how_to_play.insert(Language::Chinese, "游戏说明：".to_string());
        m.insert("minesweeper.how_to_play", how_to_play);

        let mut press_enter = HashMap::new();
        press_enter.insert(Language::English, "Press Enter to start".to_string());
        press_enter.insert(Language::Chinese, "按回车键开始".to_string());
        m.insert("minesweeper.press_enter", press_enter);

        let mut controls = HashMap::new();
        controls.insert(
            Language::English,
            "- Use arrow keys or WASD to move\n- Space to reveal a cell\n- F to flag/unflag a mine\n- P/ESC to pause\n- Yellow highlight shows current cell".to_string()
        );
        controls.insert(
            Language::Chinese,
            "- 使用方向键或WASD移动\n- 空格键翻开格子\n- F键标记/取消标记地雷\n- P/ESC暂停游戏\n- 黄色高亮显示当前选中的格子".to_string()
        );
        m.insert("minesweeper.controls", controls);

        let mut goal = HashMap::new();
        goal.insert(
            Language::English,
            "Win by either:\n- Revealing all safe cells\n- Correctly flagging all mines".to_string()
        );
        goal.insert(
            Language::Chinese,
            "胜利条件：\n- 翻开所有安全格子\n- 或正确标记所有地雷".to_string()
        );
        m.insert("minesweeper.goal", goal);
        
        m
    };
}