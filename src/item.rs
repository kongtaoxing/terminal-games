#[derive(Clone, Copy, PartialEq)]
pub enum ItemType {
    Gold,    // 金子
    Stone,   // 石头
    Nothing, // 空
}

#[derive(Clone)]
pub struct Item {
    pub x: f32,
    pub y: f32,
    pub item_type: ItemType,
    pub value: i32,
    pub size: f32,
    pub weight: f32,
} 