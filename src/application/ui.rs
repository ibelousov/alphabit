use fltk::{*, draw::*};
use crate::application::animation;
use crate::settings::*;
use crate::field::*;

pub fn draw_direction(j:i32, i:i32, direction:Direction)
{
    set_line_style(LineStyle::Solid, 2);

    let x1 = j * CELL_SIZE + 20;
    let y1 = i * CELL_SIZE + 20 + OFFSET_Y;

    match direction {
        Direction::LeftToRight => {
            draw_line(x1, y1, (j + 1) * CELL_SIZE, i * CELL_SIZE + 20 + OFFSET_Y);
        }
        Direction::RightToLeft => {
            draw_line(x1, y1, j * CELL_SIZE, i * CELL_SIZE + 20 + OFFSET_Y);
        }
        Direction::UpToDown => {
            draw_line(x1, y1, j * CELL_SIZE + 20, (i + 1) * CELL_SIZE + OFFSET_Y);
        }
        Direction::DownToUp => {
            draw_line(x1, y1,j * CELL_SIZE + 20, i * CELL_SIZE + OFFSET_Y);
        }
        Direction::DownRightToUpLeft => {
            draw_line(x1, y1, j * CELL_SIZE, i * CELL_SIZE + OFFSET_Y);
        }
        Direction::UpRightToDownLeft => {
            draw_line(x1, y1,j * CELL_SIZE, (i+1) * CELL_SIZE + OFFSET_Y);
        }
        Direction::DownLeftToUpRight => {
            draw_line(x1, y1, (j+1) * CELL_SIZE, i * CELL_SIZE + OFFSET_Y);
        }
        Direction::UpLeftToDownRight => {
            draw_line(x1, y1,(j + 1) * CELL_SIZE, (i + 1) * CELL_SIZE + OFFSET_Y);
        }
        Direction::None => {}
    }
}

pub fn draw_empty_ceil(x: i32, y: i32, bg: enums::Color) {
    draw_rect_fill(x * CELL_SIZE + 1, y * CELL_SIZE + 1 + OFFSET_Y, 38, 38, bg);
}

pub fn draw_ceil(x: i32, y: i32, bg: enums::Color, fg: enums::Color, letter: char) {
    draw_empty_ceil(x,y,bg);
    set_draw_color(fg);
    draw_text( &format!("{}", letter), x * CELL_SIZE + 15, y * CELL_SIZE + 25 + OFFSET_Y);
}

pub fn draw_ceil_direction(x: i32, y: i32, before_direction: Direction, after_direction: Direction) {
    set_draw_color(enums::Color::White);
    draw_direction(x,y,before_direction);
    draw_direction(x,y,after_direction);
}

pub fn draw_scores(scores: i32) {
    let red = animation::ColorGenerator::get_color_component(
        500, 2000,120, 180
    );
    set_font(enums::Font::Courier, 16);
    draw_rect_fill( 0,0,40 * WIDTH, OFFSET_Y,enums::Color::rgb_color(red,50, 40));
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("ОЧКИ: {:0>5}", scores), 15, 25);
}

pub fn draw_longest_word(word: String) {
    let blue = animation::ColorGenerator::get_color_component(
        1000, 2000,190, 230
    );
    draw_rect_fill(150,0,300, OFFSET_Y, enums::Color::rgb_color(0,140,blue));
    set_draw_color(enums::Color::rgb_color(255,255,255));
    if word.len() > 0 {
        draw_text( &format!("{:^25}", word.to_uppercase()), 175, 25);
    } else {
        draw_text( &format!("{:^25}", "-"), 175, 25);
    }
}

pub fn draw_finish_button() {
    let green = animation::ColorGenerator::get_color_component(
        1000, 2000,180, 220
    );
    draw_rect_fill(450,0,150, OFFSET_Y, enums::Color::rgb_color(50,green,160));
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("{}", "ФИНИШ"), 500, 25);
}

pub fn draw_bg() {
    let red = animation::ColorGenerator::get_color_component(
        500, 2000,60, 80
    );
    let green = animation::ColorGenerator::get_color_component(
        250, 6000,80, 100
    );
    let blue = animation::ColorGenerator::get_color_component(
        1000, 3000,100, 120
    );
    draw_rect_fill(
        0,
        OFFSET_Y,
        WIDTH * CELL_SIZE,
        HEIGHT * CELL_SIZE,
        enums::Color::rgb_color(red,green, blue)
    );
}