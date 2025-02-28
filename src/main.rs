use macroquad::prelude::*;
use std::env;

const TAM_CELULA: f32 = 32.0;
const LARGURA_PADRAO: i8 = 16;

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
