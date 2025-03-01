use macroquad::prelude::*;
use std::env;

const TAM_CELULA: f32 = 32.0;
const LARGURA_PADRAO: i8 = 16;
const INTERVALO_ATUALIZACAO: f64 = 0.3;

type Celula = (i8, i8);

const CIMA:(i8, i8)       = (0, -1);
const BAIXO:(i8, i8)      = (0, 1);
const ESQUERDA:(i8, i8)   = (-1, 0);
const DIREITA:(i8, i8)    = (1, 0);

struct Cobra {
    cabeca: Celula,
    corpo: Vec<Celula>,
    proximo: Celula,
}

impl Cobra{
    fn new(largura: i8) -> Self {
        let mut corpo = Vec::new();

        for _ in 0..4{
            corpo.push((largura/2, largura/2));
        }

        Cobra {
            cabeca: corpo[0],
            corpo,
            proximo: CIMA,
        }
    }

    fn mover(&mut self) {
        let (cx, cy) = self.cabeca;
        let (px, py) = self.proximo;
        let proxima: Celula = (cx + px, cy + py);

        self.corpo.insert(0, proxima);
        self.cabeca = proxima;
        self.corpo.pop();
    }
}

#[macroquad::main("Simple Snake")]
async fn main() {
    // setup do jogo

    let args: Vec<String> = env::args().collect();

    let largura_grade = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(LARGURA_PADRAO);

    let mut cobra = Cobra::new(largura_grade);

    let mut ultimo_frame = get_time();

    loop {
        clear_background(GRAY);

        // Desenha as linhas horizontais e verticais da grade
        for i in 0..=largura_grade {
            let l = i as f32 * TAM_CELULA;
            draw_line(l, 0.0, l, TAM_CELULA * largura_grade as f32, 2.0, WHITE);
            draw_line(0.0, l, TAM_CELULA * largura_grade as f32, l, 2.0, WHITE);
        }

        let frame_atual = get_time();
        if frame_atual - ultimo_frame > INTERVALO_ATUALIZACAO{
            cobra.mover();
            ultimo_frame = frame_atual;
        }

        for (x, y) in &cobra.corpo {
            draw_rectangle(
                *x as f32 * TAM_CELULA,
                *y as f32 * TAM_CELULA,
                TAM_CELULA,
                TAM_CELULA,
                GREEN
            );
        }

        next_frame().await
    }
}
