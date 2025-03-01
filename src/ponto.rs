use macroquad::prelude::*;
use ::rand::{rng, Rng};

type Celula = (i8, i8);

pub struct Ponto {
	posicao: Celula,
}

impl Ponto{
    pub fn new(largura: i8,) -> Self {
        let mut rng = rng();
		let x = rng.random_range(0..largura);
		let y = rng.random_range(0..largura);
        Ponto {
			posicao: (x, y),
        }
    }

	pub fn desenhar(&self, tamanho_celula: f32){
		let (x, y) = self.posicao;
		draw_circle(
			x as f32 * tamanho_celula + tamanho_celula/2.0,
			y as f32 * tamanho_celula + tamanho_celula/2.0,
			tamanho_celula/3.0,
			GOLD,
		);
	}

	pub fn get_posicao(&self) -> Celula {
		self.posicao
	}
}