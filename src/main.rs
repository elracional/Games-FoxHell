#[macro_use]
extern crate stdweb;                                                        // Importación de librería web estándar

mod canvas;                                                                 // Módulo local para el uso simplificado de un canvas
mod direction;                                                              // Módulo local para definir una enumeración de posibles direcciones
mod projectile;                                                             // Módulo local para el uso de sprites de tipo proyectil
mod character;                                                              // Módulo local para la lógica que rodea al jugador
mod sprite;                                                                 // Módulo local para definir un sprite compuesto de 3 u64 y la lista de colores
mod bitmap;                                                                 // Módulo local para almacenar un mapa de bits y las operaciones que se pueden realizar sobre él
mod state;                                                                  // Módulo local para definir una enumeración de posibles estados
mod gamemap;                                                                // Módulo local para definir el mapa del juego y las operaciones que se pueden realizar sobre él
mod gamestate;                                                              // Módulo local para definir una enumeración de posibles estados del juego

use canvas::Canvas;                                                         // Importación de la clase Canvas
use character::Character;                                                   // Importación de la clase Character
use gamestate::GameState;                                                   // Importación de la enumerción GameState

use stdweb::traits::*;                                                      // Importación de reglas básicas de la librería estándar web para WASM
use stdweb::web::{event::{KeyDownEvent, KeyUpEvent}, IEventTarget};         // Importación de los eventos de JavaScript

use std::cell::RefCell;                                                     // Importación de métodos de la clase RefCell
use std::rc::Rc;                                                            // Importación de la clase Rc (RefCell)

fn main() {                                                                 // Función main() que contiene la aplicación
    stdweb::initialize();                                                   // Función que inicia la aplicación web

    let canvas = Canvas::new("#canvas", 20, 10);                            // Se crea una instancia de la clase Canvas para almacenar la pantalla de juego
    let graph = Canvas::new("#graph", 10, 5);                               // Se crea una instancia de la clase Canvas para almacenar la máquina de estados y sus transiciones
    let character = Rc::new(RefCell::new(Character::new(20, 10)));          // Se crea una instancia de la clase Character para crear al jugador, se utiliza con RefCell para poder usarlo sin problemas de Ownership de Rust

    stdweb::web::document().add_event_listener( {                           // Se usa esta función para agregar un escuchador de eventos, en este caso de crean los eventos de tipo KeyDown
        let character = character.clone();                                  // Se obtiene un duplicado del objeto del jugador para usarlo sin problemas
        move |event: KeyDownEvent| {                                        // Se declara el evento KeyDown
            let key = event.key();                                          // Se obtiene la tecla que fue presionada
            if character.borrow_mut().game_state == GameState::Play {       // Se evalúa que el juego esté corriendo normalmente
                match key.as_ref() {                                        // Se usa match a modo de SWITCH (en C) para comparara qué tecla fue presionada
                    "ArrowLeft" => {                                        // En caso de haber sido la flecha izquierda
                        character.borrow_mut().set_left_key(true);              // se asigna true a la variable que indica si la tecla fue presionada
                        character.borrow_mut().arrow_left_down();               // y se indica que la tecla fue presionada
                    },
                    "ArrowRight" => {                                       // En caso de haber sido la flecha derecha
                        character.borrow_mut().set_right_key(true);              // se asigna true a la variable que indica si la tecla fue presionada
                        character.borrow_mut().arrow_right_down();               // y se indica que la tecla fue presionada
                    },
                    "ArrowDown"=> character.borrow_mut().arrow_down_down(), // En caso de haber sido abajo sólo se indica que se presionó
                    "ArrowUp"=> character.borrow_mut().arrow_up_down(),     // Mismo caso para la tecla de arriba
                    _ => {},                                                // Cualquier otra tecla se ignora
                };
            }
        }
    });

    stdweb::web::document().add_event_listener( {                           // Mismo caso que la definición del escuchador de eventos anterior
        let character = character.clone();                                  // con al diferencia de que en este se declaran los eventos
        move |event: KeyUpEvent| {                                          // de tipo KeyUp (cuando se levanta una tecla).
            let key = event.key();
            match key.as_ref() {
                "ArrowLeft" => {
                    character.borrow_mut().set_left_key(false);
                    character.borrow_mut().arrow_left_up();
                },
                "ArrowRight" => {
                    character.borrow_mut().set_right_key(false);
                    character.borrow_mut().arrow_right_up();
                },
                "ArrowDown" => character.borrow_mut().arrow_down_up(),
                " " => character.borrow_mut().start(),
                _ => {},
            };
        }
    });
                                                                            // El loop del juego, donde se corren las funciones que ejecutan el juego
    fn game_loop(character: Rc<RefCell<Character>>, canvas: Rc<Canvas>, graph: Rc<Canvas>, time: u32){
        stdweb::web::set_timeout(move || {                                  // Se define como función recursiva liberando memoria por periodos de tiemp
                game_loop(                                                  // La función se vuelve a llamar
                    character.clone(),                                      // Recibe un duplicado del jugador
                    canvas.clone(),                                         // Un duplicado del canvas del juego
                    graph.clone(),                                          // Un dulpicado del canvas del status
                    time                                                    // Y el lapso de tiempo a esperar para la siguiente iteración en milisegundos
                );
                character.borrow().draw(&canvas);                           // Cada iteración se imprime el juego en el canvas
                character.borrow_mut().update(&graph);                      // Y se ejecuta un update en el jugador, donde se actualiza el status
            },
            time,                                                           // La iteración se repite en intervalos definidos por time
        );
    }

    game_loop(character, Rc::new(canvas), Rc::new(graph), 45);              // Primera llamada a la función game_loop(...);

    stdweb::event_loop();                                                   // Se ejecuta un loop de la aplicación web
}
