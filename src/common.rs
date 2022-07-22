#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Login,
    Connecting,
    Lobby,
}

pub struct LoginState {
    pub name: String,
    pub room: String,
}
