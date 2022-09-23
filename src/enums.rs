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

#[derive(Debug, Clone)]
pub enum ButtonState {
    Normal,
    Hover,
    Pressed
}
