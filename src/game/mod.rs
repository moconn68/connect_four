/// Contains components and logic for the game board and interactions.
pub mod board;
/// Game state components and logic.
pub mod state;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GameMode {
    #[default]
    Ai,
    Local,
    Online,
}

impl TryFrom<i8> for GameMode {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ai),
            2 => Ok(Self::Local),
            3 => Ok(Self::Online),
            _ => Err(()),
        }
    }
}
