#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameResource {
    #[default]
    Wood,
    Stone
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum VillagerState {
    Idle,
    Loading,
    Carrying,
    Working
}

#[derive(Debug, Clone, PartialEq)]
pub enum VillagerType {
    Hauler,
    Builder
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuildingType {
    Warehouse,
    House
}
