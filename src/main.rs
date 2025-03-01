use macroquad::prelude::*;
use std::env;
mod cobra;
use cobra::Cobra;

const TAMANHO_CELULA: f32 = 32.0;
const RAIO_PADRAO: i8 = 5;
const INTERVALO_ATUALIZACAO: f64 = 0.1;
const TAMANHO_INICIAL: i8 = 4;

#[macroquad::main("Simple Snake")]
async fn main() {
    // setup do jogo

    let args: Vec<String> = env::args().collect();

    let raio = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(RAIO_PADRAO);
    let largura_grade = raio * 2 + 1;

    let mut cobra = Cobra::new(largura_grade, TAMANHO_INICIAL);

    let mut ultimo_frame = get_time();

    loop {
        clear_background(GRAY);

        cobra.capturar_movimento();

        // Desenha as linhas horizontais e verticais da grade
        for i in 0..=largura_grade {
            let l = i as f32 * TAMANHO_CELULA;
            draw_line(l, 0.0, l, TAMANHO_CELULA * largura_grade as f32, 0.5, WHITE);
            draw_line(0.0, l, TAMANHO_CELULA * largura_grade as f32, l, 0.5, WHITE);
        }

        let frame_atual = get_time();
        if frame_atual - ultimo_frame > INTERVALO_ATUALIZACAO{
            cobra.mover();
            ultimo_frame = frame_atual;
        }

        cobra.desenhar(TAMANHO_CELULA);

        next_frame().await
    }
}
