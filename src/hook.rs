#[derive(PartialEq)]
pub enum HookState {
    Idle,      // 钩子待机
    Extending, // 钩子伸出
    Retracting, // 钩子收回
} 