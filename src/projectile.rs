/*
 * En este módulo se definen varias estructuras de datos y una clase para el uso de los proyectiles del juego.
 * 
 * Aquí se tiene una enumeración con las posibles direcciones de un proyectil. Una estructura de datos para
 * determinar la posición del proyectil (coordenadas y dirección en que se disparó).
 * 
 * Después se tiene la clase proyectil, esta se encarga de definir si un proyectil es hostil (roca de lava)
 * o es pacífico (manzana), esto hace que la dirección en que se dispara varíe, siendo horizontal para los
 * pacíficos y vertical hacia abajo para los hostiles.
 * 
 * Se dispara un proyectil con el método shoot_at y ahí se define si es hostil (kill = true) o no (kill = false).
 * En ese método se genera el proyectil con el sprite indicado.
 * 
 * El otro método se encarga de mantener la trayectoria de un pryectil dependiendo de su posición (coordenadas y
 * dirección).
 * 
 */

use crate::sprite::Sprite;
use stdweb::unstable::TryInto;

#[derive(Debug)]
pub enum ProjectileDirection {
	Down,
	Left,
	Right,
	DownLeft,
	DownRight,
}

#[derive(Debug)]
pub struct ProjectileCoord {
	pub x: i32,
	pub y: i32,
	pub dir: ProjectileDirection,
}

#[derive(Debug)]
pub struct Projectile {
	pub coord: ProjectileCoord,
	pub spte: Sprite,
	x_max: i32,
	y_max: i32,
}

impl Projectile {
	pub fn new(spte: &Sprite) -> Projectile {
		let spte = spte.deref();
		let coord = ProjectileCoord{
			x: 0,
			y: 0,
			dir: ProjectileDirection::Down,
		};

		Projectile {
			coord,
			spte,
			x_max: 0,
			y_max: 0,
		}
	}

	pub fn shoot_at(&mut self, x: u32, y: u32, x_max: u32, y_max: u32, kill: bool) {
		let x_max = x_max as i32;
		let y_max = y_max as i32;
		let x = x as i32;
		let y = y as i32;
		let rand_dir:i8 = js!(return Math.floor(Math.random() * 3))
			.try_into()
			.unwrap();
		let rand_y:i32 = js!(return Math.floor(Math.random() * 6))
			.try_into()
			.unwrap();
		self.x_max = x_max;
		self.y_max = y_max;
		self.coord = if kill {
			match rand_dir {
				1 => {
					let mut x_in=0;
					let mut y_in=0;
					for i in 0..10 {
						if x-i<1 || y-i<2 {
							x_in = x-i;
							y_in = y-i;
							break;
						}
					}
					ProjectileCoord{
					x: x_in,
					y: y_in,
					dir: ProjectileDirection::DownRight,
					}
				},
				2 => {
					let mut x_in=0;
					let mut y_in=0;
					for i in 0..10 {
						if x+i==x_max-1 || y-i<2 {
							x_in = x+i;
							y_in = y-i;
							break;
						}
					}
					ProjectileCoord{
					x: x_in,
					y: y_in,
					dir: ProjectileDirection::DownLeft,
					}
				},
				_ => ProjectileCoord{
					x: x,
					y: 1,
					dir: ProjectileDirection::Down,
				},
			}
		} else {
			match rand_dir {
				1 => ProjectileCoord{
					x: 0,
					y: rand_y + 2,
					dir: ProjectileDirection::Right,
				},
				_ => ProjectileCoord{
					x: x_max-1,
					y: rand_y + 2,
					dir: ProjectileDirection::Left,
				},
			}
		}
	}

	pub fn next_coord(&mut self) -> bool {
		let mut result = true;
		match self.coord.dir {
			ProjectileDirection::Down => {
				self.coord.y = if self.coord.y+1==self.y_max { result=false; self.coord.y } else { self.coord.y+1 };
			},
			ProjectileDirection::Left => {
				self.coord.x = if self.coord.x-1<0 { result=false; self.coord.x } else { self.coord.x-1 };
			},
			ProjectileDirection::Right => {
				self.coord.x = if self.coord.x+1>self.x_max { result=false; self.coord.x } else { self.coord.x+1 };
			},
			ProjectileDirection::DownLeft => {
				self.coord.x = if self.coord.x-1<0 { result=false; self.coord.x } else { self.coord.x-1 };
				self.coord.y = if self.coord.y+1==self.y_max { result=false; self.coord.y } else { self.coord.y+1 };
			},
			ProjectileDirection::DownRight => {
				self.coord.x = if self.coord.x+1>self.x_max { result=false; self.coord.x } else { self.coord.x+1 };
				self.coord.y = if self.coord.y+1==self.y_max { result=false; self.coord.y } else { self.coord.y+1 };
			},
		};
		result
	}
}