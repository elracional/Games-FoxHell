/*
 * Este módulo contiene la clase BitMap en la cual se define el mapa de bits que conforma un sprite.
 * El mapa de bits consiste en una matriz 8x8 (para un sprite 8bits) en la cual cada posición indica un index
 * que posteriormente se usará para saber qué color se debe insertar en cada una de las posiciones de la
 * matriz, debido a la estructura de datos que se utiliza se pueden usar 8 opciones (3bits) de color, donde
 * el 0 indica vacío o sin color, y los números del 1 al 7 indican el index del color que se debe usar para
 * ese campo de la matriz.
 * 
 * Esta clase se encarga de la compresión de una matriz 8x8 a la asiganción de los valores en variables de tipo
 * u64. Esto se diseñó así porque en una matriz 8x8 se tienen 64 valores, por lo que si queremos guardar
 * valores que sólo necesiten 3 bits tenemos 8 posibilidades. Cada uno de los bits que reprsentan un sólo valor
 * de la matriz se van almacenando en las variables u64 que tienen 64 bits de almacenamiento disponibles, esto
 * nos permite guardar los bits más significativos de cada uno de las celdas de la matriz en la primer variable
 * u64, el siguiente bit de cada número en la segunda variable u64 y el bit menos significativo en el otro u64.
 * Para leerlo se construye la matriz llenando cada celda con los bits que se van leyendo de las variables u64.
 * 
 * Esta clase permite tanto comprimir una matriz como descomprimirla.
 * 
 * Aquí también se definen métodos para invertir el lado del mapa de bits y así invertir un sprite de dirección
 * y también para hacer un giro hacia la derecha, estas operaciones se aplican sobre la matriz 8x8.
 */

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct BitMap( pub [[u8; 8]; 8] );

impl fmt::Display for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "___")?;
        for line in self.0.iter() {
            for bit in line.iter() {
                write!(f, "{}, ", bit)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "---")
    }
}

impl BitMap {
    pub fn new() -> BitMap {
        BitMap([[0; 8]; 8])
    }

    pub fn compress_map(self) -> (u64, u64, u64) {
        let mut a:u64=0;
        let mut b:u64=0;
        let mut c:u64=0;
        let mut i=0;

        for line in self.0.iter() {
            for color in line.iter() {
                a |= ((*color as u64&4)>>2)<<i;
                b |= ((*color as u64&2)>>1)<<i;
                c |= (*color as u64&1)<<i;
                i+=1;
            }
        }

        (a, b, c)
    }

    pub fn from_compress(compressed: (u64, u64, u64)) -> BitMap {
        let (a, b, c) = compressed;
        let mut map: [[u8; 8]; 8] = [[0; 8]; 8];
        let mut i=0;

        for line in 0..8 {
            for bit in 0..8 {
                let temp:u8 = ((((a&(1<<i))>>i)<<2) | (((b&(1<<i))>>i)<<1) | (c&(1<<i))>>i) as u8;
                map[line][bit] = temp;
                i+=1;
            }
        }

        BitMap(map)
    }

    pub fn invert_side(&mut self) {
        for i in 0..8 {
            for j in 0..4 {
                let temp = self.0[i][j];
                self.0[i][j] = self.0[i][(7-j) as usize];
                self.0[i][(7-j) as usize] = temp;
            }
        }
    }

    pub fn right_rotate(&mut self) {
        let mut temp = BitMap::new();
        for i in 0..8 {
            for j in 0..8 {
                temp.0[i][j] = self.0[7-j][i];
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                self.0[i][j] = temp.0[i][j];
            }
        }
    }
}
