use macroquad::{shapes::{draw_circle, draw_circle_lines, draw_line}, text::draw_text, window::{clear_background, next_frame, screen_height, screen_width}};

use crate::{conformation::Conformation, pheromones::Pheromones, protein::AminoAcid};

impl Conformation {
    pub async fn draw(&self, iteration: u16, best: f64, pheromones: Option<&Pheromones>) {
        clear_background(macroquad::color::WHITE);

        draw_text(&format!("Iteração: {}", iteration), 10.0, 25.0, 30.0, macroquad::color::BLACK);
        draw_text(&format!("Melhor: {}", best), 13.0, 45.0, 30.0, macroquad::color::BLACK);

        let mut loc = (1, 0); /* Inicializa posição do agente */
        let mut v = (1, 0);   /* Inicializa velocidade do agente */

        let scale = 30.0;
        let screen_center = (screen_width() / 2.0, screen_height() / 2.0);
        let multi = 0.3; /* Tamanho dos círculos */

        // Desenha aminoácidos fixos
        self.draw_amino_acid(self.protein[0], (0, 0), screen_center, scale, multi);
        self.draw_amino_acid(self.protein[1], (1, 0), screen_center, scale, multi);

        // Desenha linha que liga aminoácidos fixos
        self.draw_line((0, 0), (1, 0), screen_center, scale);

        for (i, direction) in self.conformation.iter().enumerate() {
            if let Some(direction) = direction {
                v = Conformation::get_new_velocity(v, *direction); /* Calcula nova velocidade */
                
                let new_loc = (loc.0 + v.0, loc.1 + v.1); /* Calcula nova posição do agente */
                
                self.draw_line(loc, new_loc, screen_center, scale);
                self.draw_amino_acid(self.protein[i + 2], new_loc, screen_center, scale, multi);
                
                loc = new_loc;
            }
        }

        pheromones.map(|p| p.draw());

        next_frame().await;
    }

    fn draw_amino_acid(
        &self,
        amino_acid: AminoAcid,
        loc: (i32, i32),
        screen_center: (f32, f32),
        scale: f32,
        multi: f32
    ) {
        if amino_acid == AminoAcid::Hydrophobic {
            draw_circle(
                screen_center.0 + loc.0 as f32 * scale,
                screen_center.1 - loc.1 as f32 * scale, // inverter y para cima no grid
                scale * multi + 0.1,
                macroquad::color::BLACK
            );
        }
        
        draw_circle_lines(
            screen_center.0 + loc.0 as f32 * scale,
            screen_center.1 - loc.1 as f32 * scale, // inverter y para cima no grid
            scale * multi,
            2.0,
            macroquad::color::BLACK
        );
    }

    fn draw_line(&self, start: (i32, i32), end: (i32, i32), screen_center: (f32, f32), scale: f32) {
        draw_line(
            screen_center.0 + start.0 as f32 * scale,
            screen_center.1 - start.1 as f32 * scale,
            screen_center.0 + end.0 as f32 * scale,
            screen_center.1 - end.1 as f32 * scale,                            
            2.0,
            macroquad::color::BLACK
        );
    }
}