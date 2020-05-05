/*
 * En este módulo se definen todas las funciones que trabajan sobre un canvas
 * ya sea para imprimir el juego o el status en el canvas del gráfo, aquí es
 * donde se lleva a cabo la impresión de los sprites de su versión comprimida
 * en variables u64 con la intención de reducir el espacio que ocupan en memoria.
 * Todas las funciones están adaptadas al tamaño del canvas, y se hacen cálculos
 * para conseguir el mejor ajuste posible.
 */

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

use crate::sprite::Sprite;
use crate::state::State;
use crate::gamemap::GameMap;

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        Canvas {
            canvas,
            ctx,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn draw_graph(&self, state: State, status: String) {
        self.clear_all("white");

        self.ctx.set_fill_style_color("dodgerblue");
        self.ctx.fill_rect(130.0, 20.0, 30.0, 30.0);
        self.ctx.fill_rect(230.0, 20.0, 30.0, 30.0);
        self.ctx.fill_rect(80.0, 80.0, 30.0, 30.0);
        self.ctx.fill_rect(280.0, 80.0, 30.0, 30.0);
        self.ctx.fill_rect(130.0, 140.0, 30.0, 30.0);
        self.ctx.fill_rect(230.0, 140.0, 30.0, 30.0);

        self.ctx.set_fill_style_color("limegreen");
        match state {
            State::Stand => {
                self.ctx.fill_rect(125.0, 15.0, 40.0, 40.0);
            },
            State::Jump(_dir) => {
                self.ctx.fill_rect(225.0, 15.0, 40.0, 40.0);
            },
            State::Walk(_dir) => {
                self.ctx.fill_rect(75.0, 75.0, 40.0, 40.0);
            },
            State::Crouch => {
                self.ctx.fill_rect(275.0, 75.0, 40.0, 40.0);
            },
            State::Fall(_dir) => {
                self.ctx.fill_rect(125.0, 135.0, 40.0, 40.0);
            },
            State::Meteor => {
                self.ctx.fill_rect(225.0, 135.0, 40.0, 40.0);
            },
        }

        self.ctx.set_fill_style_color("black");

        self.ctx.set_font("Bold 15px Arial");
        self.ctx.fill_text("ST", 135.0, 40.0, Some(20.0));
        self.ctx.fill_text("JM", 235.0, 40.0, Some(20.0));
        self.ctx.fill_text("WK", 85.0, 100.0, Some(20.0));
        self.ctx.fill_text("CR", 285.0, 100.0, Some(20.0));
        self.ctx.fill_text("FL", 135.0, 160.0, Some(20.0));
        self.ctx.fill_text("MT", 235.0, 160.0, Some(20.0));

        self.ctx.set_font("30px Arial");
        self.ctx.fill_text(status.as_ref(), 50.0, 250.0, Some(390.0));
    }

    pub fn draw_sprite(&self, x: u32, y: u32, spte: &Sprite) {
        let x = x * self.scaled_width;
        let y = y * self.scaled_height;
        let width = self.scaled_width as f64 / 8.0;
        let height = self.scaled_height as f64 / 8.0;

        for (i, row) in spte.map.0.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                if *pixel==0 {
                    continue;
                }
                self.ctx.set_fill_style_color(&spte.colorset[(*pixel-1) as usize]);
                self.ctx.fill_rect(
                    x as f64 + j as f64 * width,
                    y as f64 + i as f64 * width,
                    width,
                    height
                );
            }
        }
    }

    pub fn draw_map(&self, map: &GameMap) {
        self.clear_all(
            if map.bg_alter { map.bg_temp.as_ref() } else { map.bg.as_ref() }
        );
        for (i, row) in map.map.iter().enumerate() {
            for (j, s) in row.iter().enumerate() {
                if *s==0 {
                    continue;
                }
                let x = j as u32;
                let y = i as u32;
                self.draw_sprite(x, y, &map.sprites[(*s-1) as usize]);
            }
        }
    }

    pub fn clear_all(&self, bg: &str) {
        self.ctx.set_fill_style_color(bg);
        self.ctx.fill_rect(
            0.0,
            0.0,
            (self.width * self.scaled_width) as f64,
            (self.height * self.scaled_height) as f64,
        );
    }
}
