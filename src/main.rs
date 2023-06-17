// #![windows_subsystem = "windows"]

use chess::{
    board::Board,
    pieces::{Piece, Player, Position},
};
use druid::piet::{ImageFormat, InterpolationMode};
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, LocalizedString, Rect, WindowDesc};
use image::io::Reader as ImageReader;
use std::fs::read;
use std::path::Path;

const WINDOW_SIZE: f64 = 800.0;
const BOARD_SIZE: usize = 816;
const PIECE_SIZE: usize = 102;

struct CustomWidget {
    board: Board,
}

impl CustomWidget {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    fn get_image_file(&self, position: Position) -> Option<String> {
        if let Some(piece) = self.board.get_piece(position) {
            let name = match piece {
                Piece::Rook(..) => "Rook",
                Piece::Bishop(..) => "Bishop",
                Piece::Pawn(..) => "Pawn",
                Piece::Knight(..) => "Knight",
                Piece::King(..) => "King",
                Piece::Queen(..) => "Queen",
            };
            let player = match piece.get_player() {
                Player::White => "white",
                Player::Black => "black",
            };
            let background = match (position.x + position.y) % 2 {
                0 => "Light",
                1 => "Dark",
                _ => "",
            };
            Some("images/".to_owned() + player + name + background + ".png")
        } else {
            None
        }
    }

    fn draw_background(&self, ctx: &mut PaintCtx) {
        let image_data = get_image("images/board.png".to_string());
        let image = ctx
            .make_image(BOARD_SIZE, BOARD_SIZE, &image_data, ImageFormat::Rgb)
            .unwrap();
        ctx.draw_image(
            &image,
            Rect::new(0.0, 0.0, WINDOW_SIZE, WINDOW_SIZE),
            InterpolationMode::Bilinear,
        );
    }

    fn draw_square(&self, ctx: &mut PaintCtx, position: Position) {
        if let Some(image_file) = self.get_image_file(position) {
            // let image_data = get_image(image_file);
            let image_data = get_image(image_file);
            let image = ctx
                .make_image(PIECE_SIZE, PIECE_SIZE, &image_data, ImageFormat::Rgb)
                .unwrap();
            let x0 = WINDOW_SIZE / 8.0 * (position.x as f64);
            let y0 = WINDOW_SIZE / 8.0 * (position.y as f64);
            ctx.draw_image(
                &image,
                Rect::new(x0, y0, x0 + WINDOW_SIZE / 8.0, y0 + WINDOW_SIZE / 8.0),
                InterpolationMode::Bilinear,
            );
        }
    }
}

impl Widget<String> for CustomWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut String, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &String,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            let size = Size::new(WINDOW_SIZE, WINDOW_SIZE);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        self.draw_background(ctx);

        for x in 0..8 {
            for y in 0..8 {
                self.draw_square(ctx, Position { x, y });
            }
        }
    }
}

pub fn main() {
    let window = WindowDesc::new(|| CustomWidget::new())
        .title(LocalizedString::new("Chess"))
        .window_size((WINDOW_SIZE, WINDOW_SIZE));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch("Druid + Piet".to_string())
        .expect("launch failed");
}

fn get_image(file_name: String) -> Vec<u8> {
    let bytes = read(file_name).unwrap();
    let img = ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    img.into_bytes()
}
