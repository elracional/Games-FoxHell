/*
 * Se define una enumeración con la lista de los posibles estados en los que se puede encontrar el jugador, en
 * este caso los estados Walk, Jump y Fall tienen un sub-estado que es la dirección, esta se define con el 
 * módulo direction, y es por que estando en esos estados se puede ir en diferentes direcciones.
 */

use crate::direction::Direction;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum State {
    Stand,
    Walk(Direction),
    Jump(Direction),
    Fall(Direction),
    Meteor,
    Crouch,
}
