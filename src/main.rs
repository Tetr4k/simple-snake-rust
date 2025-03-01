use macroquad::prelude::*;
use std::env;

const TAM_CELULA: f32 = 32.0;
const LARGURA_PADRAO: i8 = 16;

type Celula = (i8, i8);

const CIMA:(i8, i8)       = (0, 1);
const BAIXO:(i8, i8)      = (0, -1);
const ESQUERDA:(i8, i8)   = (1, 0);
const DIREITA:(i8, i8)    = (-1, 0);

struct Cobra {
    cabeca: Celula,
    corpo: Vec<Celula>,
    proximo: Celula,
}

impl Cobra{
    fn new(largura: i8) -> Self {
        Cobra {
            cabeca: (largura/2, largura/2),
            corpo: Vec::new(),
            proximo: CIMA,
        }
    }

    fn mover(&mut self) {
        let (cx, cy) = self.cabeca;
        let (px, py) = self.proximo;
        let proxima: Celula = (cx + px, cy + py);
        self.corpo.push(proxima);
        self.corpo.pop();
    }
}

#[macroquad::main("Simple Snake")]
async fn main() {
    // setup do jogo

    let args: Vec<String> = env::args().collect();

    let largura_grade = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(LARGURA_PADRAO);

    loop {
        clear_background(GRAY);

        // Desenha as linhas horizontais e verticais da grade
        for i in 0..=largura_grade {
            let l = i as f32 * TAM_CELULA;
            draw_line(l, 0.0, l, TAM_CELULA * largura_grade as f32, 2.0, WHITE);
            draw_line(0.0, l, TAM_CELULA * largura_grade as f32, l, 2.0, WHITE);
        }

        next_frame().await
    }
}
