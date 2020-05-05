/*
 * Clase que contiene un mapa de juego usando una lista de sprites y un mapa de bits indicando qué sprite debemos
 * poner en cada uno de los espacios del mapa creado. Este se imprime al fondo del canvas.
 */

use crate::sprite::Sprite;

#[derive(Debug)]
pub struct GameMap {
    pub map: [[u8; 20]; 10],
    pub sprites: Vec<Sprite>,
    i: u8,
    pub bg: String,
    pub bg_temp: String,
    pub bg_alter: bool,
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            map: [[0; 20]; 10],
            sprites: Vec::new(),
            i: 0,
            bg: String::new(),
            bg_temp: String::new(),
            bg_alter: false,
        }
    }

    pub fn from_text(text: String) -> GameMap {
        let mut map = GameMap::new();

        let mut state=0;
        for line in text.lines() {
            match line {
                "BackgroundStart"                                               => { state=3 },
                "BackTempStart"                                                 => { state=4 },
                "MapStart"                                                      => { state=1 },
                "SpriteStart"                                                   => { state=2 },
                "" | "MapEnd" | "SpriteEnd" | "BackgroundEnd" | "BackTempEnd"   => { state=0 },
                _ => {
                    match state {
                        1 => { map.add_line(line) },
                        2 => { map.add_sprite(Sprite::from_line(line.to_owned())) },
                        3 => { map.bg.push_str(line) },
                        4 => { map.bg_temp.push_str(line) },
                        _ => (),
                    }
                }
            }
        }
        map
    }

    pub fn add_line(&mut self, line: &str) {
        for (i, c) in line.chars().enumerate() {
            if c >= '0' && c <= '9' {
                self.map[self.i as usize][i] = (c as u8 - '0' as u8) as u8;
            } else if c >= 'A' && c <= 'Z' {
                self.map[self.i as usize][i] = ((c as u8) - ('A' as u8) + (10 as u8) ) as u8;
            }
        }
        self.i+=1;
    }

    pub fn add_sprite(&mut self, spte: Sprite) {
        self.sprites.push(spte);
    }

}
