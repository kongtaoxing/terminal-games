use super::Language;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TETRIS_TRANSLATIONS: HashMap<&'static str, HashMap<Language, String>> = {
        let mut m = HashMap::new();

        let mut tetris_title = HashMap::new();
        tetris_title.insert(Language::English, "Tetris".to_string());
        tetris_title.insert(Language::Chinese, "俄罗斯方块".to_string());
        m.insert("tetris.tetris_title", tetris_title);

        let mut score = HashMap::new();
        score.insert(Language::English, "Score:".to_string());
        score.insert(Language::Chinese, "分数：".to_string());
        m.insert("tetris.score", score);

        let mut game_over = HashMap::new();
        game_over.insert(Language::English, "Game Over!".to_string());
        game_over.insert(Language::Chinese, "游戏结束！".to_string());
        m.insert("tetris.game_over", game_over);

        let mut welcome_to = HashMap::new();
        welcome_to.insert(Language::English, "Welcome to".to_string());
        welcome_to.insert(Language::Chinese, "欢迎来到".to_string());
        m.insert("tetris.welcome_to", welcome_to);

        let mut how_to_play = HashMap::new();
        how_to_play.insert(Language::English, "How to Play:".to_string());
        how_to_play.insert(Language::Chinese, "游戏说明：".to_string());
        m.insert("tetris.how_to_play", how_to_play);

        let mut move_horizontal = HashMap::new();
        move_horizontal.insert(
            Language::English,
            "1. Use LEFT/RIGHT Arrow or A/D to move block horizontally".to_string(),
        );
        move_horizontal.insert(
            Language::Chinese,
            "1. 使用左右箭头键或A/D键左右移动方块".to_string(),
        );
        m.insert("tetris.move_horizontal", move_horizontal);

        let mut speed_up = HashMap::new();
        speed_up.insert(
            Language::English,
            "2. Use DOWN Arrow or S to speed up block descent".to_string(),
        );
        speed_up.insert(
            Language::Chinese,
            "2. 使用向下箭头键或S键加速方块下降".to_string(),
        );
        m.insert("tetris.speed_up", speed_up);

        let mut rotate = HashMap::new();
        rotate.insert(
            Language::English,
            "3. Use UP Arrow or W to rotate block".to_string(),
        );
        rotate.insert(
            Language::Chinese,
            "3. 使用向上箭头键或W键旋转方块".to_string(),
        );
        m.insert("tetris.rotate", rotate);

        let mut hard_drop = HashMap::new();
        hard_drop.insert(
            Language::English,
            "4. Press SPACE to hard drop block".to_string(),
        );
        hard_drop.insert(Language::Chinese, "4. 按空格键瞬间放下方块".to_string());
        m.insert("tetris.hard_drop", hard_drop);

        let mut clear_lines = HashMap::new();
        clear_lines.insert(
            Language::English,
            "5. Clear lines to score points".to_string(),
        );
        clear_lines.insert(Language::Chinese, "5. 清除行以获得分数".to_string());
        m.insert("tetris.clear_lines", clear_lines);

        let mut one_line = HashMap::new();
        one_line.insert(Language::English, "1 line cleared: 100 points".to_string());
        one_line.insert(Language::Chinese, "清除1行：100分".to_string());
        m.insert("tetris.one_line", one_line);

        let mut two_lines = HashMap::new();
        two_lines.insert(Language::English, "2 lines cleared: 300 points".to_string());
        two_lines.insert(Language::Chinese, "清除2行：300分".to_string());
        m.insert("tetris.two_lines", two_lines);

        let mut three_lines = HashMap::new();
        three_lines.insert(Language::English, "3 lines cleared: 500 points".to_string());
        three_lines.insert(Language::Chinese, "清除3行：500分".to_string());
        m.insert("tetris.three_lines", three_lines);

        let mut four_lines = HashMap::new();
        four_lines.insert(Language::English, "4 lines cleared: 800 points".to_string());
        four_lines.insert(Language::Chinese, "清除4行：800分".to_string());
        m.insert("tetris.four_lines", four_lines);

        let mut game_ends = HashMap::new();
        game_ends.insert(
            Language::English,
            "Game ends when new blocks can't enter the playing field".to_string(),
        );
        game_ends.insert(
            Language::Chinese,
            "当新方块无法进入游戏区域时游戏结束".to_string(),
        );
        m.insert("tetris.game_ends", game_ends);

        let mut quit_control = HashMap::new();
        quit_control.insert(Language::English, "Q: Return to main menu".to_string());
        quit_control.insert(Language::Chinese, "Q：返回主菜单".to_string());
        m.insert("tetris.quit_control", quit_control);

        let mut press_enter = HashMap::new();
        press_enter.insert(Language::English, "Press ENTER to start!".to_string());
        press_enter.insert(Language::Chinese, "按回车键开始".to_string());
        m.insert("tetris.press_enter", press_enter);

        let mut pause_game = HashMap::new();
        pause_game.insert(
            Language::English,
            "Press P or ESC to pause Game".to_string(),
        );
        pause_game.insert(Language::Chinese, "按P或ESC键暂停游戏".to_string());
        m.insert("tetris.pause_game", pause_game);

        let mut game_paused = HashMap::new();
        game_paused.insert(Language::English, "Game Paused".to_string());
        game_paused.insert(Language::Chinese, "游戏暂停".to_string());
        m.insert("tetris.game_paused", game_paused);

        let mut press_p_to_resume = HashMap::new();
        press_p_to_resume.insert(Language::English, "Press P or ESC to resume".to_string());
        press_p_to_resume.insert(Language::Chinese, "按P或ESC键继续".to_string());
        m.insert("tetris.press_p_to_resume", press_p_to_resume);

        let mut restart = HashMap::new();
        restart.insert(
            Language::English,
            "Press R to restart after game over".to_string(),
        );
        restart.insert(Language::Chinese, "失败后按R键重新开始".to_string());
        m.insert("tetris.restart", restart);

        m
    };
}
