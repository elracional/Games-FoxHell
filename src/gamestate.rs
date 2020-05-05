/*
 * Enumeración que indica los posibles estados en que puede estar la ejecución del juego.
 */

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Start,
    Play,
    GameOver,
}
