#![windows_subsystem = "windows"]

mod logic;

use eframe::egui;
use logic::{CellContent, CellState, GameStatus, Grid};
use std::time::{Duration, Instant};

fn main() -> eframe::Result<()> {
    let icon_bytes = include_bytes!("../assets/bob-omb.ico");
    let icon_image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .to_rgba8();
    let (icon_width, icon_height) = icon_image.dimensions();
    let icon_rgba = icon_image.into_raw();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0]) // Initial size, will be resized
            .with_icon(std::sync::Arc::new(egui::IconData {
                rgba: icon_rgba,
                width: icon_width,
                height: icon_height,
            })),
        ..Default::default()
    };

    eframe::run_native(
        "Mario Minesweeper",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(Minesweeper::new(cc)))
        }),
    )
}

struct Minesweeper {
    grid: Grid,
    start_time: Option<Instant>,
    elapsed: Duration,
    difficulty: Difficulty,
    has_flagged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn settings(&self) -> (usize, usize, usize) {
        match self {
            Difficulty::Easy => (9, 9, 10),
            Difficulty::Medium => (16, 16, 40),
            Difficulty::Hard => (30, 16, 99),
        }
    }
}

// Visual constants
const COLOR_BOARD: egui::Color32 = egui::Color32::from_rgb(180, 180, 180);
const COLOR_UNREVEALED: egui::Color32 = egui::Color32::from_rgb(180, 180, 180);
const COLOR_REVEALED: egui::Color32 = egui::Color32::from_rgb(165, 165, 165);
const COLOR_HIGHLIGHT: egui::Color32 = egui::Color32::from_rgb(245, 245, 245);
const COLOR_SHADOW: egui::Color32 = egui::Color32::from_rgb(90, 90, 90);
const COLOR_GRID_LINE: egui::Color32 = egui::Color32::from_rgb(130, 130, 130);

impl Minesweeper {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let difficulty = Difficulty::Easy;
        let (w, h, m) = difficulty.settings();
        let slf = Minesweeper {
            grid: Grid::new(w, h, m),
            start_time: None,
            elapsed: Duration::default(),
            difficulty,
            has_flagged: false,
        };
        let size = slf.desired_size();
        cc.egui_ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
        slf
    }

    fn desired_size(&self) -> egui::Vec2 {
        let (w, h, _) = self.difficulty.settings();
        let cell_size = 24.0;
        let header_height = 60.0;
        let padding = 20.0;
        let grid_w = w as f32 * cell_size;
        let grid_h = h as f32 * cell_size;
        let total_w = grid_w + padding * 2.0;
        let total_h = grid_h + header_height + padding;
        // Top menu + Bottom status
        egui::vec2(total_w, total_h + 52.0)
    }

    fn reset(&mut self) {
        let (w, h, m) = self.difficulty.settings();
        self.grid = Grid::new(w, h, m);
        self.start_time = None;
        self.elapsed = Duration::default();
        self.has_flagged = false;
    }

    fn set_difficulty(&mut self, d: Difficulty, ctx: &egui::Context) {
        if self.difficulty != d {
            self.difficulty = d;
            self.reset();
            let size = self.desired_size();
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
        }
    }

    fn draw_7seg_digit(&self, painter: &egui::Painter, rect: egui::Rect, digit: char, color: egui::Color32) {
        let w = rect.width();
        let h = rect.height();
        let thickness = w * 0.15;
        let gap = 1.0;

        let segments = match digit {
            '0' => [true, true, true, true, true, true, false],
            '1' => [false, true, true, false, false, false, false],
            '2' => [true, true, false, true, true, false, true],
            '3' => [true, true, true, true, false, false, true],
            '4' => [false, true, true, false, false, true, true],
            '5' => [true, false, true, true, false, true, true],
            '6' => [true, false, true, true, true, true, true],
            '7' => [true, true, true, false, false, false, false],
            '8' => [true, true, true, true, true, true, true],
            '9' => [true, true, true, true, false, true, true],
            '-' => [false, false, false, false, false, false, true],
            _ => [false, false, false, false, false, false, false],
        };

        let draw_seg = |idx: usize, r: egui::Rect| {
            if segments[idx] {
                painter.rect_filled(r, 0.5, color);
            } else {
                painter.rect_filled(r, 0.5, color.gamma_multiply(0.05));
            }
        };

        draw_seg(0, egui::Rect::from_min_max(rect.left_top() + egui::vec2(gap, 0.0), rect.left_top() + egui::vec2(w - gap, thickness)));
        draw_seg(1, egui::Rect::from_min_max(rect.right_top() + egui::vec2(-thickness, gap), rect.right_top() + egui::vec2(0.0, h / 2.0 - gap / 2.0)));
        draw_seg(2, egui::Rect::from_min_max(rect.right_bottom() + egui::vec2(-thickness, -h / 2.0 + gap / 2.0), rect.right_bottom() + egui::vec2(0.0, -gap)));
        draw_seg(3, egui::Rect::from_min_max(rect.left_bottom() + egui::vec2(gap, -thickness), rect.left_bottom() + egui::vec2(w - gap, 0.0)));
        draw_seg(4, egui::Rect::from_min_max(rect.left_bottom() + egui::vec2(0.0, -h / 2.0 + gap / 2.0), rect.left_bottom() + egui::vec2(thickness, -gap)));
        draw_seg(5, egui::Rect::from_min_max(rect.left_top() + egui::vec2(0.0, gap), rect.left_top() + egui::vec2(thickness, h / 2.0 - gap / 2.0)));
        draw_seg(6, egui::Rect::from_center_size(rect.center(), egui::vec2(w - gap * 2.0, thickness)));
    }

    fn draw_digital_display(&self, painter: &egui::Painter, val: String, rect: egui::Rect) {
        painter.rect_filled(rect, 0.0, egui::Color32::BLACK);
        let digit_w = (rect.width() - 10.0) / 3.0;
        let digit_h = rect.height() - 10.0;
        
        let mut chars: Vec<char> = val.chars().collect();
        while chars.len() < 3 {
            chars.insert(0, ' ');
        }

        for (i, &c) in chars.iter().enumerate() {
            let digit_rect = egui::Rect::from_min_size(
                rect.left_top() + egui::vec2(5.0 + i as f32 * digit_w, 5.0),
                egui::vec2(digit_w - 2.0, digit_h)
            );
            self.draw_7seg_digit(painter, digit_rect, c, egui::Color32::RED);
        }
    }

    fn draw_bevel(&self, painter: &egui::Painter, rect: egui::Rect, width: f32, raised: bool) {
        let (tl, br) = if raised {
            (COLOR_HIGHLIGHT, COLOR_SHADOW)
        } else {
            (COLOR_SHADOW, COLOR_HIGHLIGHT)
        };

        let stroke_tl = egui::Stroke::new(width, tl);
        let stroke_br = egui::Stroke::new(width, br);

        painter.line_segment([rect.left_bottom(), rect.left_top()], stroke_tl);
        painter.line_segment([rect.left_top(), rect.right_top()], stroke_tl);
        painter.line_segment([rect.right_top(), rect.right_bottom()], stroke_br);
        painter.line_segment([rect.right_bottom(), rect.left_bottom()], stroke_br);
    }

    fn draw_flag(&self, painter: &egui::Painter, rect: egui::Rect) {
        let center = rect.center();
        let s = rect.width() * 0.6;
        let pole_x = center.x - s * 0.15;
        
        // Pole
        painter.line_segment(
            [egui::pos2(pole_x, center.y - s * 0.35), egui::pos2(pole_x, center.y + s * 0.4)],
            egui::Stroke::new(s * 0.1, egui::Color32::BLACK)
        );
        // Flag
        let path = vec![
            egui::pos2(pole_x, center.y - s * 0.35),
            egui::pos2(pole_x + s * 0.5, center.y - s * 0.1),
            egui::pos2(pole_x, center.y + s * 0.15),
        ];
        painter.add(egui::Shape::convex_polygon(path, egui::Color32::RED, egui::Stroke::NONE));
        // Base
        painter.line_segment(
            [egui::pos2(pole_x - s * 0.2, center.y + s * 0.4), egui::pos2(pole_x + s * 0.2, center.y + s * 0.4)],
            egui::Stroke::new(s * 0.1, egui::Color32::BLACK)
        );
    }

    fn draw_mine(&self, painter: &egui::Painter, rect: egui::Rect, exploded: bool) {
        let center = rect.center();
        let r = rect.width() * 0.3;
        
        if exploded {
            painter.circle_filled(center, r * 1.5, egui::Color32::from_rgba_unmultiplied(255, 0, 0, 100));
        }

        painter.circle_filled(center, r, egui::Color32::BLACK);
        // Spikes
        for i in 0..8 {
            let angle = i as f32 * std::f32::consts::PI / 4.0;
            let p1 = center + egui::vec2(angle.cos(), angle.sin()) * r;
            let p2 = center + egui::vec2(angle.cos(), angle.sin()) * r * 1.5;
            painter.line_segment([p1, p2], egui::Stroke::new(1.5, egui::Color32::BLACK));
        }
        // Shine
        painter.circle_filled(center + egui::vec2(-r*0.4, -r*0.4), r*0.25, egui::Color32::WHITE);
    }

    fn draw_smiley(&self, painter: &egui::Painter, rect: egui::Rect, status: GameStatus, any_cell_pressed: bool) {
        let center = rect.center();
        let radius = rect.width() / 2.0 * 0.8;
        painter.circle_filled(center, radius, egui::Color32::YELLOW);
        painter.circle_stroke(center, radius, egui::Stroke::new(1.0, egui::Color32::BLACK));

        match status {
            GameStatus::Playing => {
                if any_cell_pressed {
                    painter.circle_filled(center + egui::vec2(-radius * 0.35, -radius * 0.3), radius * 0.12, egui::Color32::BLACK);
                    painter.circle_filled(center + egui::vec2(radius * 0.35, -radius * 0.3), radius * 0.12, egui::Color32::BLACK);
                    painter.circle_stroke(center + egui::vec2(0.0, radius * 0.35), radius * 0.2, egui::Stroke::new(1.0, egui::Color32::BLACK));
                } else {
                    painter.circle_filled(center + egui::vec2(-radius * 0.35, -radius * 0.3), radius * 0.1, egui::Color32::BLACK);
                    painter.circle_filled(center + egui::vec2(radius * 0.35, -radius * 0.3), radius * 0.1, egui::Color32::BLACK);
                    let p1 = center + egui::vec2(-radius * 0.4, radius * 0.2);
                    let p2 = center + egui::vec2(0.0, radius * 0.45);
                    let p3 = center + egui::vec2(radius * 0.4, radius * 0.2);
                    painter.line_segment([p1, p2], egui::Stroke::new(1.2, egui::Color32::BLACK));
                    painter.line_segment([p2, p3], egui::Stroke::new(1.2, egui::Color32::BLACK));
                }
            }
            GameStatus::Won => {
                let glass_w = radius * 0.4;
                let glass_h = radius * 0.3;
                painter.rect_filled(egui::Rect::from_center_size(center + egui::vec2(-radius * 0.35, -radius * 0.25), egui::vec2(glass_w, glass_h)), 1.0, egui::Color32::BLACK);
                painter.rect_filled(egui::Rect::from_center_size(center + egui::vec2(radius * 0.35, -radius * 0.25), egui::vec2(glass_w, glass_h)), 1.0, egui::Color32::BLACK);
                painter.line_segment([center + egui::vec2(-radius * 0.2, -radius * 0.25), center + egui::vec2(radius * 0.2, -radius * 0.25)], egui::Stroke::new(1.0, egui::Color32::BLACK));
                let p1 = center + egui::vec2(-radius * 0.4, radius * 0.2);
                let p2 = center + egui::vec2(0.0, radius * 0.45);
                let p3 = center + egui::vec2(radius * 0.4, radius * 0.2);
                painter.line_segment([p1, p2], egui::Stroke::new(1.2, egui::Color32::BLACK));
                painter.line_segment([p2, p3], egui::Stroke::new(1.2, egui::Color32::BLACK));
            }
            GameStatus::Lost => {
                let eye_r = radius * 0.15;
                let eye1 = center + egui::vec2(-radius * 0.35, -radius * 0.3);
                painter.line_segment([eye1 - egui::vec2(eye_r, eye_r), eye1 + egui::vec2(eye_r, eye_r)], egui::Stroke::new(1.2, egui::Color32::BLACK));
                painter.line_segment([eye1 - egui::vec2(eye_r, -eye_r), eye1 + egui::vec2(eye_r, -eye_r)], egui::Stroke::new(1.2, egui::Color32::BLACK));
                let eye2 = center + egui::vec2(radius * 0.35, -radius * 0.3);
                painter.line_segment([eye2 - egui::vec2(eye_r, eye_r), eye2 + egui::vec2(eye_r, eye_r)], egui::Stroke::new(1.2, egui::Color32::BLACK));
                painter.line_segment([eye2 - egui::vec2(eye_r, -eye_r), eye2 + egui::vec2(eye_r, -eye_r)], egui::Stroke::new(1.2, egui::Color32::BLACK));
                let p1 = center + egui::vec2(-radius * 0.4, radius * 0.45);
                let p2 = center + egui::vec2(0.0, radius * 0.2);
                let p3 = center + egui::vec2(radius * 0.4, radius * 0.45);
                painter.line_segment([p1, p2], egui::Stroke::new(1.2, egui::Color32::BLACK));
                painter.line_segment([p2, p3], egui::Stroke::new(1.2, egui::Color32::BLACK));
            }
        }
    }
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(start) = self.start_time {
            self.elapsed = start.elapsed();
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Easy").clicked() { self.set_difficulty(Difficulty::Easy, ctx); }
                if ui.button("Medium").clicked() { self.set_difficulty(Difficulty::Medium, ctx); }
                if ui.button("Hard").clicked() { self.set_difficulty(Difficulty::Hard, ctx); }
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                match self.grid.status {
                    GameStatus::Won => {
                        let msg = if self.has_flagged { "Game Cleared" } else { "Game Cleared (NF)" };
                        ui.label(egui::RichText::new(msg).strong());
                    }
                    GameStatus::Lost => {
                        ui.label(egui::RichText::new("Game Over").color(egui::Color32::RED));
                    }
                    GameStatus::Playing => {
                        if self.start_time.is_some() { ui.label("Playing..."); } else { ui.label("Ready"); }
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let cell_size = 24.0;
            let header_height = 60.0;
            let padding = 20.0;
            let grid_w = self.grid.width as f32 * cell_size;
            let grid_h = self.grid.height as f32 * cell_size;
            let total_w = grid_w + padding * 2.0;
            let total_h = grid_h + header_height + padding;

            let available_rect = ui.available_rect_before_wrap();
            let start_x = available_rect.left() + (available_rect.width() - total_w) / 2.0;
            let start_y = available_rect.top() + (available_rect.height() - total_h) / 2.0;

            let board_rect = egui::Rect::from_min_size(egui::pos2(start_x, start_y), egui::vec2(total_w, total_h));
            let painter = ui.painter();
            painter.rect_filled(board_rect, 0.0, COLOR_BOARD);
            self.draw_bevel(&painter, board_rect, 3.0, true);

            let header_rect = egui::Rect::from_min_size(egui::pos2(start_x + padding, start_y + 10.0), egui::vec2(grid_w, header_height - 20.0));
            self.draw_bevel(&painter, header_rect.expand(3.0), 3.0, false);

            let mine_count = (self.grid.mine_count as i32 - self.grid.flagged_count() as i32).max(-99).min(999);
            self.draw_digital_display(&painter, format!("{}", mine_count), egui::Rect::from_min_size(egui::pos2(start_x + padding + 5.0, start_y + 15.0), egui::vec2(60.0, 30.0)));
            self.draw_digital_display(&painter, format!("{}", self.elapsed.as_secs().min(999)), egui::Rect::from_min_size(egui::pos2(start_x + padding + grid_w - 65.0, start_y + 15.0), egui::vec2(60.0, 30.0)));

            let grid_rect = egui::Rect::from_min_size(egui::pos2(start_x + padding, start_y + header_height), egui::vec2(grid_w, grid_h));
            let any_cell_pressed = ui.input(|i| i.pointer.primary_down()) && ui.rect_contains_pointer(grid_rect);

            let smiley_rect = egui::Rect::from_center_size(egui::pos2(start_x + padding + grid_w / 2.0, start_y + 30.0), egui::vec2(30.0, 30.0));
            let smiley_response = ui.interact(smiley_rect, ui.id().with("smiley"), egui::Sense::click());
            if smiley_response.clicked() { self.reset(); }
            painter.rect_filled(smiley_rect, 0.0, COLOR_UNREVEALED);
            self.draw_bevel(&painter, smiley_rect, 2.0, !smiley_response.is_pointer_button_down_on());
            self.draw_smiley(&painter, smiley_rect, self.grid.status, any_cell_pressed);

            self.draw_bevel(&painter, grid_rect.expand(3.0), 3.0, false);

            for y in 0..self.grid.height {
                for x in 0..self.grid.width {
                    let rect = egui::Rect::from_min_size(egui::pos2(start_x + padding + x as f32 * cell_size, start_y + header_height + y as f32 * cell_size), egui::vec2(cell_size, cell_size));
                    let response = ui.interact(rect, ui.id().with(("cell", x, y)), egui::Sense::click());

                    if response.clicked() && self.grid.status == GameStatus::Playing {
                        if self.start_time.is_none() { self.start_time = Some(Instant::now()); }
                        self.grid.reveal(x, y);
                        if self.grid.status != GameStatus::Playing { self.start_time = None; }
                    }
                    if response.secondary_clicked() && self.grid.status == GameStatus::Playing {
                        self.grid.toggle_flag(x, y);
                        self.has_flagged = true;
                    }

                    let cell = self.grid.get_cell(x, y);
                    match cell.state {
                        CellState::Revealed => {
                            let bg = if self.grid.exploded_mine == Some((x, y)) { egui::Color32::RED } else { COLOR_REVEALED };
                            painter.rect_filled(rect, 0.0, bg);
                            painter.rect_stroke(rect, 0.0, egui::Stroke::new(0.5, COLOR_GRID_LINE), egui::StrokeKind::Inside);
                            match cell.content {
                                CellContent::Mine => self.draw_mine(&painter, rect, self.grid.exploded_mine == Some((x, y))),
                                CellContent::Empty(n) if n > 0 => {
                                    painter.text(rect.center(), egui::Align2::CENTER_CENTER, n.to_string(), egui::FontId::proportional(18.0), number_color(n));
                                }
                                _ => {}
                            }
                        }
                        CellState::VictoryRevealed => {
                            painter.rect_filled(rect, 0.0, COLOR_REVEALED);
                            painter.rect_stroke(rect, 0.0, egui::Stroke::new(0.5, COLOR_GRID_LINE), egui::StrokeKind::Inside);
                            self.draw_flag(&painter, rect);
                        }
                        CellState::Flagged => {
                            painter.rect_filled(rect, 0.0, COLOR_UNREVEALED);
                            self.draw_bevel(&painter, rect, 2.0, true);
                            self.draw_flag(&painter, rect);
                        }
                        _ => {
                            let mut fill = COLOR_UNREVEALED;
                            if response.hovered() { 
                                fill = egui::Color32::from_rgb(200, 200, 200); 
                            }
                            painter.rect_filled(rect, 0.0, fill);
                            let pressed = response.is_pointer_button_down_on();
                            self.draw_bevel(&painter, rect, 2.0, !pressed);
                        }
                    }
                }
            }
        });
    }
}

fn number_color(n: u8) -> egui::Color32 {
    match n {
        1 => egui::Color32::from_rgb(0, 0, 255),
        2 => egui::Color32::from_rgb(0, 128, 0),
        3 => egui::Color32::from_rgb(255, 0, 0),
        4 => egui::Color32::from_rgb(0, 0, 128),
        5 => egui::Color32::from_rgb(128, 0, 0),
        6 => egui::Color32::from_rgb(0, 128, 128),
        7 => egui::Color32::BLACK,
        8 => egui::Color32::from_rgb(128, 128, 128),
        _ => egui::Color32::BLACK,
    }
}
