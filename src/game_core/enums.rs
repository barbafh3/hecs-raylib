#[derive(Debug, Clone)]
pub enum GameResource {
    Wood,
    Stone
}

#[derive(Debug, Clone)]
pub enum CollisionType {
    Body,
    Trigger,
    All
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Toggled
}
