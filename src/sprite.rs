/*
 * En esta clase se hace uso de la clase BitMap para definir el mapa de bits de un sprite y de una lista/vector 
 * para almacenar los colores que contiene el sprite, cada uno de estos colores se determina con el index indicado
 * en el mapa de bits.
 * 
 * Un sprite puede ser generado completamente con su versión en una línea, que contiene el nombre del sprite, la
 * lista de colores y los tres números u64 que conforman el mapa de bits, este se construye con BitMap.
 * 
 * En esta clase también se definen métodos para costruir un sprite a partir de una línea que se importa de un archivo,
 * y también se define un método para generar un diccionario usando como index el nombre del sprite obtenido de la
 * línea importada y conteniedo un sprite por campo.
 * 
 * El método deref() sirve para crear un duplicado de un sprite sin afectar a las reglas de Rust sobre Ownership.
 */

use std::fmt;
use std::string::String;
use std::collections::HashMap;
use crate::bitmap::BitMap;

#[derive(Debug)]
pub struct Sprite {
    pub map: BitMap,
    pub colorset: [String; 7],
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.map.0.iter() {
            for n in line {
                write!(f, "{}, ", n)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

impl Sprite {
    pub fn new(colors: Vec<String>, compressed_map: (u64, u64, u64)) -> Sprite {
        let mut colorset = [String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new()
        ];
        for (i, color) in colors.iter().enumerate() {
            colorset[i].push_str(color);
        }
        Sprite {
            map: BitMap::from_compress(compressed_map),
            colorset,
        }
    }

    pub fn deref(&self) -> Sprite {
        let mut colors: Vec<String> = Vec::new();
        let compressed_map = self.map.compress_map();

        for color in self.colorset.iter() {
            colors.push(color.to_string());
        }

        Sprite::new(colors, compressed_map)
    }

    pub fn from_line(line: String) -> Sprite {
        let mut colors: Vec<String> = Vec::new();
        let mut compressed_map: [u64; 3] = [0; 3];

        let mut data = line.split(":");

        data.next().unwrap();
    {
        let clrs = data.next().unwrap();
        for color in clrs.split("-") {
            colors.push(String::from(color));
        }
    }
    {
        let compress = data.next().unwrap();
        for (i, cmp) in compress.split("-").enumerate() {
            compressed_map[i] = String::from(cmp).parse::<u64>().unwrap();
        }
    }

        Sprite::new(colors, (compressed_map[0], compressed_map[1], compressed_map[2]) )
    }

    pub fn hash_from_text (text: String) -> HashMap<String, Sprite> {
        let mut hash = HashMap::new();

        for line in text.lines() {
            let key = line.split(":").next().unwrap();
            let spte = Sprite::from_line(String::from(line));

            hash.insert(String::from(key), spte);
        }
        hash
    }
}
