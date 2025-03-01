use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound_once};
use macroquad::miniquad::window::{set_window_size};
use std::env;

mod cobra;
use cobra::Cobra;

mod ponto;
use ponto::Ponto;

const TAMANHO_CELULA: f32 = 64.0;
const RAIO_PADRAO: i8 = 5;
const INTERVALO_ATUALIZACAO: f64 = 0.1;

#[macroquad::main("Simple Snake")]
async fn main() {
    set_pc_assets_folder("assets");
    
    let coin_collect = load_sound("audio/coin_collect.wav").await.unwrap();

    // setup do jogo

    let args: Vec<String> = env::args().collect();

    let raio = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(RAIO_PADRAO);
    let largura_grade = raio * 2 + 1;

    let tamanho = largura_grade as f32 * TAMANHO_CELULA;
    set_window_size(tamanho as u32, tamanho as u32);

    let mut cobra = Cobra::new(largura_grade);
    let mut ponto = Ponto::new(largura_grade);

    let mut ultimo_frame = get_time();

    loop {
        clear_background(DARKGRAY);

        cobra.capturar_movimento();

        // Desenha as linhas horizontais e verticais da grade
        for i in 0..=largura_grade {
            let l = i as f32 * TAMANHO_CELULA;
            draw_line(l, 0.0, l, TAMANHO_CELULA * largura_grade as f32, 0.5, WHITE);
            draw_line(0.0, l, TAMANHO_CELULA * largura_grade as f32, l, 0.5, WHITE);
        }

        let frame_atual = get_time();
        if frame_atual - ultimo_frame > INTERVALO_ATUALIZACAO{
            cobra.mover(largura_grade);
            ultimo_frame = frame_atual;
        }

        if cobra.get_cabeca() == ponto.get_posicao() {
            play_sound_once(&coin_collect);
            cobra.cresce();
            ponto = Ponto::new(largura_grade);
        }

        cobra.desenhar(TAMANHO_CELULA);
        ponto.desenhar(TAMANHO_CELULA);

        next_frame().await
    }
}
