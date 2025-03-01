use macroquad::prelude::*;

type Celula = (i8, i8);

pub struct Cobra {
    cabeca: Celula,
    pub corpo: Vec<Celula>,
    pub proximo: Celula,
}

impl Cobra{
    pub fn new(largura: i8, tamanho_inicial: i8) -> Self {
        let mut corpo = Vec::new();

        for _ in 0..tamanho_inicial{
            corpo.push((largura/2, largura/2));
        }

        Cobra {
            cabeca: corpo[0],
            corpo,
            proximo: (1, 0),
        }
    }

    pub fn mover(&mut self) {
        let (cx, cy) = self.cabeca;
        let (px, py) = self.proximo;
        let proxima: Celula = (cx + px, cy + py);

        self.corpo.insert(0, proxima);
        self.cabeca = proxima;
        self.corpo.pop();
    }

    pub fn capturar_movimento(&mut self) {
        const CIMA:Celula       = (0, -1);
        const BAIXO:Celula      = (0, 1);
        const ESQUERDA:Celula   = (-1, 0);
        const DIREITA:Celula    = (1, 0);

        if is_key_pressed(KeyCode::Up) && self.proximo != BAIXO {
            self.proximo = CIMA;
        }
        if is_key_pressed(KeyCode::Down) && self.proximo != CIMA {
            self.proximo = BAIXO;
        }
        if is_key_pressed(KeyCode::Left) && self.proximo != DIREITA {
            self.proximo = ESQUERDA;
        }
        if is_key_pressed(KeyCode::Right) && self.proximo != ESQUERDA {
            self.proximo = DIREITA;
        }
    }

    pub fn desenhar(&self, tamanho_celula: f32) {
        for (x, y) in &self.corpo {
            draw_rectangle(
                *x as f32 * tamanho_celula,
                *y as f32 * tamanho_celula,
                tamanho_celula,
                tamanho_celula,
                GREEN,
            );
        }
    }
}