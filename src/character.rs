/*
 * El objetivo del videojuego es recolectar las manzana que cruzan horizontalmente el mapa de juego
 * mientras se evitan las rocas de lava que hacen daño al jugador, al final del juego se muestra
 * la cantidad de puntos que se obtuvieron al recolectar las manzanas.
 * 
 * En este módulo se desarrolla la mayor parte del videojuego, definiendo el comportamiento
 * del jugador y las condiciones del entorno. Aquí también se encuentra el mapa, los proyectiles,
 * la máquina de estados, ya que son los estados en los que puede estar el jugador durante la
 * ejecución del programa.
 * 
 * Se utiliza una estructura de coordenadas para facilitar su comparación y copia dentro de ciertas
 * codiciones que se necesitan entre el jugador y el mapa.
 * 
 * La carga de sprites se hace en tiempo de compilación puesto que es imposible leer un archivo cuando
 * el programa se encuentra en ejecución, esto viene siendo una limitación leve por parte del uso de 
 * stdweb. De ahí en fuera el comportamiento del programa es el habitual.
 * 
 * Para cargar los sprites de archivos se incluyen como referecias a string, y se crean usando la
 * clase Sprite línea por línea según la información contenida en el archivo. El archivo contiene los
 * sprites en versión comprimida, por lo que en la creación de éstos se descomprimen dando como
 * resultado un sprite imprimible por las funciones dento del módulo canvas.
 * 
 * El estado inicial del jugador es Stand y podrá cambiar siguiendo las siguientes reglas:
 *   Stand  ---     Up KeyDown     ---> Jump
 *   Stand  ---     L/R KeyDown    ---> Walk
 *   Stand  ---    Down KeyDown    ---> Crouch
 *   Jump   ---  MaxHeightReached  ---> Fall
 *   Jump   ---    Down KeyDown    ---> Meteor
 *   Fall   ---    Down KeyDown    ---> Meteor
 *   Fall   ---        Land        ---> Stand
 *   Fall   --- Land & L/R KeyDown ---> Walk
 *   Walk   ---      L/R KeyUp     ---> Stand
 *   Walk   ---     Up KeyDown     ---> Jump
 *   Walk   ---       NoFloor      ---> Fall
 *   Meteor ---        Land        ---> Crouch
 *   Crouch ---     Down KeyUp     ---> Stand
 * De cualquiera de estos estados se puede pasar a muerto y concluir la ejecución de esa sesión de juego.
 * 
 * La dinámica del juego consiste en evadir las rocas de lava que caen del cielo mientras se recogen las manzanas
 * que van pasando horizontalmente desde los lados. La cantidad de manzanas recogidas se muestra en la pantalla
 * en todo momento para estar siempre pendientes del puntaje, así mismo se muestran las vidas en el lado izquierdo.
 * 
 * Las bolas de lava se dirigen siempre al jugador y vienen siempre de arriba, arriba-izquierda o arriba-derecha,
 * en cambio las manzanas sólo aparecen de forma lateral y a alturas aleatorias.
 * 
 * Los mapas son configurables desde un archivo por lo que se puede implementar fácil un cambio en el mapa de juego.
 * Los archivos de los mapas que se muestran son mapStart.txt, map0.txt y mapEnd.txt.
 * 
 * Para generar las animaciones o acciones periódicas se utilizó un contador de iteraciones para que cada ciertos
 * intervalos de tiempo se puedan ejecutar ciertas acciones, por ejemplo, los cambios de sprite para simular
 * animaciones o el movimiento de las nubes.
 */

use crate::canvas::Canvas;                                                  // Uso de los otros
use crate::direction::Direction;                                            // módulos desarrollados
use crate::sprite::Sprite;                                                  // para completar la
use crate::gamemap::GameMap;                                                // ejecución del juego
use crate::state::State;                                                    // en secciones por
use crate::projectile::Projectile;                                          // separado para mayor
use crate::gamestate::GameState;                                            // organización.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord(u32, u32);                                                 // Estructura de coordenadas

#[derive(Debug)]
pub struct Character {                                                      // Estructura de datos del jugador que contiene toda la información útil para dar la jugabilidad al juego
    coord: Coord,
    sprites: [Vec<Sprite>; 5],
    state: State,
    pub last_state: State,
    stt_ix: usize,
    height: u32,
    width: u32,
    dir: Direction,
    last_dir: Direction,

    pub game_state: GameState,

    it: usize,
    max_it: usize,

    map_start: GameMap,
    map: GameMap,
    map_end: GameMap,

    jump_h: u8,
    max_jump_h: u8,

    left_key: bool,
    right_key: bool,
    down_key: bool,

    ices: Vec<Projectile>,
    apples: Vec<Projectile>,

    score: u16,

    lifes: u8,
}

impl Character {
    pub fn new(width: u32, height: u32) -> Character {                      // Constructor de la clase, donde se definen todos los valores necesarios para la correcta ejecución del juego
        let coord = Coord(9, 8);
        let mut sprites = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];

        let hash = Sprite::hash_from_text(String::from(include_str!("files/fox_sprites.txt")));

        let spte: Sprite = hash.get("Stand1").unwrap().deref();
        let spte2: Sprite = hash.get("Stand2").unwrap().deref();
        sprites[0].push(spte);
        sprites[0].push(spte2);

        let spte: Sprite = hash.get("Walk1").unwrap().deref();
        let spte2: Sprite = hash.get("Walk2").unwrap().deref();
        sprites[1].push(spte);
        sprites[1].push(spte2);

        let spte: Sprite = hash.get("Jump").unwrap().deref();
        sprites[2].push(spte);
        let mut spte: Sprite = hash.get("Jump").unwrap().deref();
        spte.map.right_rotate();
        sprites[2].push(spte);
        let mut spte: Sprite = hash.get("Jump").unwrap().deref();
        spte.map.right_rotate();
        spte.map.right_rotate();
        sprites[2].push(spte);
        let mut spte: Sprite = hash.get("Jump").unwrap().deref();
        spte.map.right_rotate();
        spte.map.right_rotate();
        spte.map.right_rotate();
        sprites[2].push(spte);

        let spte: Sprite = hash.get("Jump").unwrap().deref();
        sprites[2].push(spte);

        let spte: Sprite = hash.get("Meteor").unwrap().deref();
        sprites[3].push(spte);
        let mut spte: Sprite = hash.get("Meteor").unwrap().deref();
        spte.map.invert_side();
        sprites[3].push(spte);

        let spte: Sprite = hash.get("Crouch").unwrap().deref();
        sprites[4].push(spte);

        Character {
            coord,
            sprites,

            state: State::Stand,
            last_state: State::Stand,
            stt_ix: 0,

            game_state: GameState::Start,

            height,
            width,

            dir: Direction::Right,
            last_dir: Direction::Right,

            it: 0,
            max_it: 200,

            map_start: GameMap::from_text(String::from(include_str!("files/mapStart.txt"))),
            map: GameMap::from_text(String::from(include_str!("files/map0.txt"))),
            map_end: GameMap::from_text(String::from(include_str!("files/mapEnd.txt"))),

            jump_h: 0,
            max_jump_h: 4,

            left_key: false,
            right_key: false,
            down_key: false,
            
            ices: Vec::new(),
            apples: Vec::new(),

            score: 0,

            lifes: 2,
        }
    }

    pub fn change_dir(&mut self, dir: Direction) {                          // Función que cambia la dirección del personaje
        if self.last_dir != dir && dir != Direction::None {
            self.invert_side();
            self.last_dir = dir;
        }
        self.dir = dir;
    }

    pub fn update(&mut self, graph: &Canvas) {                              // Función que actualiza el proceso del juego
        if self.game_state == GameState::Start || self.game_state == GameState::GameOver {
            if self.it % 2 == 0 {
                self.map_start.sprites[(('F' as u8) - ('A' as u8) + (9 as u8) ) as usize].map.invert_side();
                self.map_end.sprites[(('O' as u8) - ('A' as u8) + (9 as u8) ) as usize].map.invert_side();
                self.map_end.sprites[7 as usize].map.invert_side();
            }
            if self.it % 10 == 0 {
                self.map_start.sprites[(('G' as u8) - ('A' as u8) + (9 as u8) ) as usize].map.invert_side();
                self.map_start.sprites[(('D' as u8) - ('A' as u8) + (9 as u8) ) as usize].map.invert_side();
                self.map_end.sprites[9 as usize].map.invert_side();
                self.map_end.sprites[(('P' as u8) - ('A' as u8) + (9 as u8) ) as usize].map.invert_side();
            }
            self.it = if self.it < self.max_it - 1 { self.it + 1 } else { 0 };
            return
        }

        self.load_hud();

        if Character::keep_track(&mut self.ices, &self.coord, false) {
            if self.lifes > 0 {
                self.lifes-=1;
            }
            else {
                self.game_state = GameState::GameOver;
            }
        }
        if Character::keep_track(&mut self.apples, &self.coord, false) {
            self.score += 1;
        }
        if !self.left_key && !self.right_key {
            self.arrow_left_up();
            self.arrow_right_up();
        }

        if self.get_state() == State::Crouch {
            if !self.down_key {
                self.set_state(State::Stand);
            }
            if self.map.map[self.coord.1 as usize][self.coord.0 as usize] == 6 && self.lifes<4 {
                self.lifes+=1;
                self.map.map[self.coord.1 as usize][self.coord.0 as usize] = 0;
            }
        }
        match self.state {
            State::Walk(dir) => {
                if self.it % 2 == 0 {
                    self.change_dir(dir);
                    let new_coord = match dir {
                        Direction::Right => Coord( (self.coord.0 + 1) % self.width, self.coord.1 ),
                        Direction::Left => Coord( (self.coord.0 + self.width - 1) % self.width, self.coord.1 ),
                        Direction::None => Coord( self.coord.0 % self.width, self.coord.1 ),
                    };
                    self.coord = new_coord;
                    if self.no_floor() {
                        self.set_state(State::Fall(dir));
                    }
                }
            },
            State::Jump(dir) => {
                if self.it % 2 == 0 {
                    self.change_dir(dir);
                    if self.jump_h < self.max_jump_h {
                        let coord_y = if self.coord.1+self.height-1<self.height { self.coord.1 } else { self.coord.1-1 };
                        let new_coord = match dir {
                            Direction::Right => Coord( (self.coord.0 + 1) % self.width, coord_y ),
                            Direction::Left => Coord( (self.coord.0 + self.width - 1) % self.width, coord_y ),
                            Direction::None => Coord( self.coord.0 % self.width, coord_y ),
                        };
                        self.coord = new_coord;
                        self.jump_h += 1;
                    } else {
                        self.set_state(State::Fall(self.dir));
                        self.jump_h = 0;
                    }
                }
            },
            State::Fall(dir) => {
                if self.it % 2 == 0 {
                    self.change_dir(dir);
                    if self.no_floor() {
                        let new_coord = match dir {
                            Direction::Right => Coord( (self.coord.0 + 1) % self.width, self.coord.1 + 1 ),
                            Direction::Left => Coord( (self.coord.0 + self.width - 1) % self.width, self.coord.1 + 1 ),
                            Direction::None => Coord( self.coord.0 % self.width, self.coord.1 + 1 ),
                        };
                        self.coord = new_coord;
                        self.set_state(State::Fall(self.dir));
                    }
                    else {
                        if self.get_dir() == Direction::None {
                            self.set_state(State::Stand);
                        }
                        else {
                            self.set_state(State::Walk(self.get_dir()));
                        }

                    }
                }
            },
            State::Meteor => {
                self.jump_h = 0;
                if self.no_floor() {
                    self.coord = Coord( self.coord.0 % self.width, self.coord.1 + 1 );
                    self.set_state(State::Meteor);
                }
                else {
                    self.set_state(State::Crouch);
                }
            }
            State::Stand => {
                if self.it % 2 == 0 && self.no_floor() {
                    self.set_state(State::Fall(Direction::None));
                }
            },
            _ => (),
        }

        for proj in self.ices.iter_mut() {
            proj.spte.map.right_rotate();
        }

        for proj in self.apples.iter_mut() {
            proj.spte.map.invert_side();
        }

        if self.it % 2 == 0 {
            self.map.sprites[6].map.invert_side();
        }

        if self.it%4==0 {
            if Character::keep_track(&mut self.ices, &self.coord, true) {
                if self.lifes > 0 {
                    self.lifes-=1;
                }
                else {
                    self.game_state = GameState::GameOver;
                }
            }

            if Character::keep_track(&mut self.apples, &self.coord, true) {
                self.score += 1;
            }
        }

        if self.it % 10 == 0 {
            self.map.sprites[2].map.invert_side();
            self.map.sprites[0].map.invert_side();
        }

        if self.it%20==0 {
            let hash = Sprite::hash_from_text(String::from(include_str!("files/fox_sprites.txt")));
            let mut lball = hash.get("LavaBall").unwrap().deref();
            lball.map.invert_side();
            let mut proj = Projectile::new(&lball);
            proj.shoot_at(self.coord.0, self.coord.1, self.width, self.height, true);
            self.ices.push(proj);

            let hash = Sprite::hash_from_text(String::from(include_str!("files/fox_sprites.txt")));
            let apl = hash.get("Apple").unwrap().deref();
            let mut proj = Projectile::new(&apl);
            proj.shoot_at(self.coord.0, self.coord.1, self.width, self.height, false);
            self.apples.push(proj);
        }

        self.it = if self.it < self.max_it - 1 { self.it + 1 } else { 0 };

        if self.last_state != self.state || self.state == State::Meteor {
            self.draw_graph(graph, self.state, self.get_transition());
        }
    }

    pub fn draw(&self, canvas: &Canvas) {                               // Función que dibuja el juego en pantalla
        if self.game_state == GameState::Start {
            canvas.draw_map(&self.map_start);
            return
        }

        if self.game_state == GameState::GameOver {
            canvas.draw_map(&self.map_end);
            return
        }

        canvas.draw_map(&self.map);

        canvas.draw_sprite(self.coord.0, self.coord.1, &self.sprites[self.stt_ix][self.it % self.sprites[self.stt_ix].len()]);

        for proj in self.apples.iter() {
            canvas.draw_sprite(proj.coord.x as u32, proj.coord.y as u32, &proj.spte);
        }

        for proj in self.ices.iter() {
            canvas.draw_sprite(proj.coord.x as u32, proj.coord.y as u32, &proj.spte);
        }
    }
                                                                        // Función que muestra el status del juego en el canvas de status
    pub fn draw_graph(&self, graph: &Canvas, state: State, status: String) {
        if self.game_state == GameState::Start || self.game_state == GameState::GameOver {
            return;
        }
        graph.draw_graph(state, status);
    }

    pub fn invert_side(&mut self) {                                     // Función que invierte la dirección de los sprites del personaje
        for spte_list in self.sprites.iter_mut() {
            for spte in spte_list {
                spte.map.invert_side();
            }
        }
    }

    pub fn no_floor(&mut self) -> bool {                                // Función para saber si hay suelo o no debajo del personaje
        self.map.map[self.coord.1 as usize + 1][self.coord.0 as usize] != 2
    }
}

impl Character {
    pub fn set_state(&mut self, state: State) {                         // Función que se encarga de realizar el cambio de estado
        self.last_state = self.state;
        self.state = state;
        self.stt_ix = match state {
            State::Stand => 0,
            State::Walk(_d) => 1,
            State::Jump(_d) => 2,
            State::Fall(_d) => 2,
            State::Meteor => 3,
            State::Crouch => 4,
        };
    }

    pub fn set_left_key(&mut self, press: bool) {                       // Función para establecer la tecla DERECHA como presionada (validación/parche)
        self.left_key = press ;
    }

    pub fn set_right_key(&mut self, press: bool) {                      // Función para establecer la tecla DERECHA como presionada (validación/parche)
        self.right_key = press ;
    }

    pub fn get_state(&self) -> State {                                  // Función para obtener el estado
        self.state
    }

    pub fn get_dir(&self) -> Direction {                                // Función para obtener la dirección
        self.dir
    }

    pub fn get_transition(&self) -> String {                            // Función que nos indica la transición de estados que se realizó
        let st1 = match self.last_state {
            State::Stand => {
                "Stand"
            },
            State::Jump(_dir) => {
                "Jump"
            },
            State::Walk(_dir) => {
                "Walk"
            },
            State::Crouch => {
                "Crouch"
            },
            State::Fall(_dir) => {
                "Fall"
            },
            State::Meteor => {
                "Meteor"
            },
        };
        let st2 = match self.state {
            State::Stand => {
                "Stand"
            },
            State::Jump(_dir) => {
                "Jump"
            },
            State::Walk(_dir) => {
                "Walk"
            },
            State::Crouch => {
                "Crouch"
            },
            State::Fall(_dir) => {
                "Fall"
            },
            State::Meteor => {
                "Meteor"
            },
        };
        let trans = format!("{} -> {}", st1, st2);
        String::from(trans)
    }

    pub fn crash(coord: &Coord, proj: &Projectile) -> bool {            // Función que compara las coordenadas del jugador con las del proyectil para detectar la colisión
        coord.0 == proj.coord.x as u32 &&
            coord.1 == proj.coord.y as u32
    }
                                                                        // Función que mantiene el curso de los proyectiles en la pantalla
    pub fn keep_track(projectiles: &mut Vec<Projectile>, coord: &Coord, move_b: bool) -> bool {
        let mut result = false;
        let mut i=0;
        let mut rem:Vec<usize> = Vec::new();
        for proj in projectiles.iter_mut() {
            if ( move_b && !proj.next_coord() ) || Character::crash(coord, proj) {
                rem.push(i);
            }
            if Character::crash(coord, proj) {
                result = true;
            }
            i+=1;
        }
        for (fix, i) in rem.iter().enumerate() {
            projectiles.remove(*i - fix);
        }
        result
    }

    pub fn load_hud(&mut self) {                                        // Función que carga la información del juego en la parte superior de la pantalla
        for i in 1..19 {
            self.map.map[0][i as usize] = 0;
        }
        for i in 0..self.lifes {
            self.map.map[0][i as usize + 1 as usize] = 5;
        }
        let s_score = String::from( format!("{:>04}", self.score) );
        for (i, c) in s_score.chars().enumerate() {
            self.map.map[0][15 as usize + i as usize] = (c as u8 - '0' as u8) + 8;
            self.map_end.map[0][8 as usize + i as usize] = (c as u8 - '0' as u8) + 26;
        }
    }
}

impl Character {
    pub fn arrow_left_down(&mut self) {                                 // Función que indica que la flecha IZQUIERDA se presionó
        if let State::Stand = self.get_state() {
            self.set_state(State::Walk(Direction::Left));
        } else if let State::Jump(_d) = self.get_state() {
            self.set_state(State::Jump(Direction::Left));
        } else if let State::Fall(_d) = self.get_state() {
            self.set_state(State::Fall(Direction::Left));
        } else if let State::Walk(_d) = self.get_state() {
            self.set_state(State::Walk(Direction::Left));
        }
    }

    pub fn arrow_right_down(&mut self) {                                // Función que indica que la flecha DERECHA se presionó
        if self.get_state() == State::Stand {
            self.set_state(State::Walk(Direction::Right));
        } else if let State::Jump(_d) = self.get_state() {
            self.set_state(State::Jump(Direction::Right));
        } else if let State::Fall(_d) = self.get_state() {
            self.set_state(State::Fall(Direction::Right));
        } else if let State::Walk(_d) = self.get_state() {
            self.set_state(State::Walk(Direction::Right));
        }
    }

    pub fn arrow_down_down(&mut self) {                                 // Función que indica que la flecha ABAJO se presionó
        self.down_key = true;
        if self.get_state() == State::Stand {
            self.set_state(State::Crouch);
        } else if let State::Jump(_d) = self.get_state() {
            self.set_state(State::Meteor);
        } else if let State::Fall(_d) = self.get_state() {
            self.set_state(State::Meteor);
        }
    }

    pub fn arrow_up_down(&mut self) {                                   // Función que indica que la flecha ARRIBA se presionó
        if self.get_state() == State::Stand {
            self.set_state(State::Jump(Direction::None));
        } else if let State::Walk(dir) = self.get_state() {
            self.set_state(State::Jump( dir ));
        }
    }

    pub fn arrow_left_up(&mut self) {                                   // Función que indica que la flecha IZQUIERDA se liberó
        if let State::Walk(dir) = self.get_state() {
            if dir == Direction::Left {
                self.set_state(State::Stand);
            }
        } else if let State::Jump(dir) = self.get_state() {
            if dir == Direction::Left {
                self.set_state(State::Jump(Direction::None));
            }
        } else if let State::Fall(dir) = self.get_state() {
            if dir == Direction::Left {
                self.set_state(State::Fall(Direction::None));
            }
        }
    }

    pub fn arrow_right_up(&mut self) {                                  // Función que indica que la flecha DERECHA se liberó
        if let State::Walk(dir) = self.get_state() {
            if dir == Direction::Right {
                self.set_state(State::Stand);
            }
        } else if let State::Jump(dir) = self.get_state() {
            if dir == Direction::Right {
                self.set_state(State::Jump(Direction::None));
            }
        } else if let State::Fall(dir) = self.get_state() {
            if dir == Direction::Right {
                self.set_state(State::Fall(Direction::None));
            }
        }
    }

    pub fn arrow_down_up(&mut self) {                                   // Función que indica que la tecla ABAJO se liberó
        self.down_key=false;
        if self.get_state() == State::Crouch {
            self.set_state(State::Stand);
        }
    }

    pub fn start(&mut self) {                                           // Función que da inicio al juego
        if self.game_state == GameState::Start {
            self.game_state = GameState::Play;

            self.state = State::Stand;
            self.last_state = State::Stand;
        }
        if self.game_state == GameState::GameOver {
            if self.dir == Direction::Left {
                self.invert_side();
            }
            self.game_state = GameState::Play;

            self.coord = Coord(9, 8);

            self.state = State::Stand;
            self.last_state = State::Stand;
            self.stt_ix = 0;

            self.dir = Direction::Right;
            self.last_dir = Direction::Right;

            self.jump_h = 0;

            self.ices.clear();
            self.apples.clear();

            self.score = 0;

            self.lifes = 2;
        }
    }
}
