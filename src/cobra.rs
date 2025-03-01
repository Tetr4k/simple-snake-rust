use macroquad::prelude::*;

type Celula = (i8, i8);

const TAMANHO_INICIAL: i8 = 4;

pub struct Cobra {
    cabeca: Celula,
    corpo: Vec<Celula>,
    proximo: Celula,
}

impl Cobra{
    pub fn new(largura: i8) -> Self {
        let mut corpo = Vec::new();

        for _ in 0..TAMANHO_INICIAL{
            corpo.push((largura/2, largura/2));
        }

        Cobra {
            cabeca: corpo[0],
            corpo,
            proximo: (1, 0),
        }
    }

    pub fn mover(&mut self, largura: i8) {
        let (cx, cy) = self.cabeca;
        let (px, py) = self.proximo;
        let proxima: Celula = (cx + px, cy + py);

        let (x, y) = proxima;

        if x == -1 || x == largura || y == -1 || y == largura || self.corpo.contains(&proxima){
            *self = Cobra::new(largura);
        }
        else{
            self.corpo.insert(0, proxima);
            self.cabeca = proxima;
            self.corpo.pop();
        }
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

    pub fn cresce(&mut self) {
        self.corpo.push(self.cabeca);
    }

    pub fn get_cabeca(&self) -> Celula {
        self.cabeca
    }
}