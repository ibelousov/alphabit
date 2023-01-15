use std::cell::{Ref};
use fltk::{*, draw::*};
use crate::application::{animation};
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

pub fn draw_leaders_table(leaders: Ref<Vec<(i32,String)>>) {

    let x = WIDTH * CELL_SIZE;
    let color = enums::Color::rgb_color(50, 90, 130);
    draw_rect_fill(x, 0, SIDEBAR_WIDTH, CELL_SIZE, color);
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("{}", "ТАБЛИЦА ЛИДЕРОВ"), WIDTH * CELL_SIZE + 80, 25);

    for (y, record) in leaders.iter().enumerate() {

        let offset = (y + 1) as u8;
        let color = if offset % 2 == 1 {
            enums::Color::rgb_color( 0x3f,0x41,0x52)
        }  else {
            enums::Color::rgb_color( 0x3f - 10,0x41 - 10,0x52 - 10)
        };
        draw_rect_fill(x, CELL_SIZE * (offset as i32), SIDEBAR_WIDTH, CELL_SIZE, color);
        set_draw_color(enums::Color::rgb_color(255,255,255));
        if record.0 > 0 {
            draw_text(&format!("{:<5} {}", record.0, record.1), WIDTH * CELL_SIZE + 10, CELL_SIZE * (offset as i32) + 25);
        } else {
            draw_text(&format!("{}", "-"), WIDTH * CELL_SIZE + 10, CELL_SIZE * (offset as i32) + 25);
        }
    }
}

pub fn draw_controls() {
    let x = WIDTH * CELL_SIZE;
    let color = enums::Color::rgb_color(50, 80, 130);
    draw_rect_fill(x, CELL_SIZE * 11, SIDEBAR_WIDTH, CELL_SIZE, color);
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("{}", "УПРАВЛЕНИЕ"), WIDTH * CELL_SIZE + 100, CELL_SIZE * 11 + 25);

    let color = enums::Color::rgb_color(50, 60, 70);
    draw_rect_fill(x, CELL_SIZE * 12, SIDEBAR_WIDTH, CELL_SIZE, color);
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("{}", "ЛЕВАЯ КН. МЫШИ - ВЫДЕЛЕНИЕ"), WIDTH * CELL_SIZE + 10, CELL_SIZE * 12 + 25);

    let color = enums::Color::rgb_color(40, 50, 60);
    draw_rect_fill(x, CELL_SIZE * 13, SIDEBAR_WIDTH, CELL_SIZE, color);
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("{}", "ПРАВАЯ КН. МЫШИ - ОТМЕНА"), WIDTH * CELL_SIZE + 10, CELL_SIZE * 13 + 25);

    let color = enums::Color::rgb_color(50, 60, 70);
    draw_rect_fill(x, CELL_SIZE * 14, SIDEBAR_WIDTH, CELL_SIZE, color);
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("{}", "ПОВТОРНЫЙ КЛИК - ВЫБОР"), WIDTH * CELL_SIZE + 10, CELL_SIZE * 14 + 25);

    let color = enums::Color::rgb_color(40, 50, 60);
    draw_rect_fill(x, CELL_SIZE * 15, SIDEBAR_WIDTH, CELL_SIZE, color);
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("{}", "ФИНИШ - ЗАКОНЧИТЬ ИГРУ"), WIDTH * CELL_SIZE + 10, CELL_SIZE * 15 + 25);
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