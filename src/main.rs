mod engine;

use std::borrow::{BorrowMut};
use engine::*;
use fltk::{app, prelude::*, *, draw::*};
use fltk::window::Window;
use fltk::enums::{Event};
use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::app::MouseButton;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    field: Field,
    position_x: i32,
    position_y: i32
}

impl ::std::default::Default for Config {
    fn default() -> Self {

        let field = Field::new(WIDTH, HEIGHT);

        field.generate();

        Self {
            field: field,
            position_x: 0,
            position_y: 0
        }
    }
}

const WIDTH: i32 = 15;
const HEIGHT: i32 = 15;
const TITLE: &str = "Альфабит";
const OFFSET_Y: i32 = 40;
const SETTINGS_NAME: &str = "alphabits-ettings";

fn draw_direction(j:i32, i:i32, direction:Direction)
{
    set_line_style(LineStyle::Solid, 2);

    match direction {
        Direction::LeftToRight => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, (j + 1) * 40, i * 40 + 20 + OFFSET_Y);
        }
        Direction::RightToLeft => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, j * 40, i * 40 + 20 + OFFSET_Y);
        }
        Direction::UpToDown => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, j * 40 + 20, (i + 1) * 40 + OFFSET_Y);
        }
        Direction::DownToUp => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y,j * 40 + 20, i * 40 + OFFSET_Y);
        }
        Direction::DownRightToUpLeft => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, j * 40, i * 40 + OFFSET_Y);
        }
        Direction::UpRightToDownLeft => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y,j * 40, (i+1) * 40 + OFFSET_Y);
        }
        Direction::DownLeftToUpRight => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, (j+1) * 40, i * 40 + OFFSET_Y);
        }
        Direction::UpLeftToDownRight => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y,(j + 1) * 40, (i + 1) * 40 + OFFSET_Y);
        }
        Direction::None => {}
    }
}

fn draw_empty_ceil(x: i32, y: i32, bg: enums::Color) {
    draw_rect_fill(x * 40 + 1, y * 40 + 1 + OFFSET_Y, 38, 38, bg);
}

fn draw_ceil(x: i32, y: i32, bg: enums::Color, fg: enums::Color, letter: char) {
    draw_empty_ceil(x,y,bg);
    set_draw_color(fg);
    draw_text( &format!("{}", letter), x * 40 + 15, y * 40 + 25 + OFFSET_Y);
}

fn draw_ceil_direction(x: i32, y: i32, before_direction: Direction, after_direction: Direction) {
    set_draw_color(enums::Color::White);
    draw_direction(x,y,before_direction);
    draw_direction(x,y,after_direction);
}

fn draw_scores(scores: i32) {
    set_font(enums::Font::Courier, 16);
    draw_rect_fill( 0,0,40 * WIDTH, OFFSET_Y,enums::Color::rgb_color(118,50, 40));
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("ОЧКИ: {:0>5}", scores), 15, 25);
}

fn draw_longest_word(word: String) {
    draw_rect_fill(150,0,300, OFFSET_Y, enums::Color::rgb_color(0,140,210));
    set_draw_color(enums::Color::rgb_color(255,255,255));
    if word.len() > 0 {
        draw_text( &format!("{:^25}", word.to_uppercase()), 175, 25);
    } else {
        draw_text( &format!("{:^25}", "-"), 175, 25);
    }
}

fn draw_finish_button() {
    draw_rect_fill(450,0,150, OFFSET_Y, enums::Color::rgb_color(50,200,160));
    set_draw_color(enums::Color::rgb_color(255,255,255));
    draw_text( &format!("{}", "ФИНИШ"), 500, 25);
}

fn draw_bg() {
    draw_rect_fill(0, OFFSET_Y, WIDTH * 40, HEIGHT * 40, enums::Color::rgb_color(40,42,54));
}

fn main() -> Result<(), confy::ConfyError> {

    let mut config: Config = confy::load(SETTINGS_NAME, None)?;

    let app = app::App::default();

    let field = Rc::new(config.field.clone());
    let field_draw = Rc::clone(&field);

    let mut wind = Window::new(config.position_x, config.position_y, WIDTH * 40, HEIGHT * 40 + OFFSET_Y, TITLE);

    let image = image::PngImage::from_data(
        include_bytes!("../assets/icon.png")
    ).unwrap();
    wind.set_icon(Some(image));

    wind.handle({
        move |f, ev| {
            match ev {
                Event::Push => {
                    if field.is_blocked() {
                       return true;
                    }

                    let (x,y) = app::event_coords();

                    if y > 5 && y < OFFSET_Y - 10 && x > 455 && x < 600 {
                        field.generate();
                        field.set_longest_word(String::new());
                        field.set_scores(0);

                        config.field = (*field).clone();

                        confy::store(SETTINGS_NAME, None, &config)
                            .expect("Не удалось сохранить настройки");

                        f.redraw();
                    }

                    if y < OFFSET_Y {
                        return true;
                    }

                    let (cell_x, cell_y) = (x / 40, (y - OFFSET_Y) / 40);

                    if app::event_mouse_button() == MouseButton::Right {
                        field.deselect();
                    } else {
                        field.try_check(cell_x, cell_y);
                    }

                    config.field = (*field).clone();
                    confy::store(SETTINGS_NAME, None, &config)
                        .expect("Не удалось сохранить настройки");

                    f.redraw();

                    true
                },
                Event::NoEvent => {

                    if config.borrow_mut().position_x != f.x() ||
                        config.borrow_mut().position_x != f.y() {
                        config.borrow_mut().position_x = f.x();
                        config.borrow_mut().position_y = f.y();
                        confy::store(SETTINGS_NAME, None, &config).expect("Не удалось сохранить настройки");
                    }

                    true
                },
                _ => false
            }
        }
    });

    wind.draw(move |_w| {

        draw_scores(field_draw.get_scores());

        draw_longest_word(field_draw.get_longest_word());

        draw_finish_button();

        let is_word = field_draw.is_word() == true;
        let is_bonus_exists = field_draw.is_bonus_exists() == true;

        draw_bg();

        for i in 0..field_draw.get_height() {
            for j in 0..field_draw.get_width() {
                let offset = (j + i * field_draw.get_width()) * 225;
                let color = if is_word {
                    enums::Color::rgb_color(
                        100,
                        engine::animation::ColorGenerator::get_color_component(
                            offset as u32,
                            4000,
                            68,
                            160
                        ),
                        100
                    )
                } else {
                    enums::Color::rgb_color(100, 100,100)
                };
                let bonus_color = enums::Color::rgb_color(
                    engine::animation::ColorGenerator::get_color_component(
                        offset as u32,
                        2000,
                        68,
                        160
                    ),
                    engine::animation::ColorGenerator::get_color_component(
                        offset as u32,
                        3222,
                        68,
                        160
                    ),
                    engine::animation::ColorGenerator::get_color_component(
                        offset as u32,
                        1000,
                        68,
                        160
                    )
                );
                let white = enums::Color::White;
                let gray_color = enums::Color::rgb_color(63,65,82);
                let almost_white = enums::Color::rgb_color(230,230,230);

                let checked_value = field_draw.is_checked(j, i);
                let is_on_the_bonus_line = field_draw.is_on_the_bonus_line(j,i);
                let letter = field_draw.get(j,i).letter;

                if checked_value > 0 {

                    if is_word && is_bonus_exists {
                        draw_ceil(j,i,bonus_color, white, letter);
                    } else {
                        draw_ceil(j,i,color,white, letter);
                    }

                    draw_ceil_direction(j,i,
                                        field_draw.get_before_direction(j,i),
                                        field_draw.get_direction(j,i)
                    );

                } else if checked_value == 0 {
                    match field_draw.get(j,i).ceil_type {
                        CeilType::Active => {
                            if is_word && is_bonus_exists && is_on_the_bonus_line {
                                draw_ceil(j, i, bonus_color, almost_white, letter);
                            } else {
                                draw_ceil(j, i, gray_color, almost_white, letter);
                            }
                        },
                        CeilType::Empty => {
                            if is_word && is_bonus_exists && is_on_the_bonus_line {
                                draw_empty_ceil(j, i, bonus_color);
                            } else {
                                draw_empty_ceil(j, i, enums::Color::rgb_color(63, 65, 82));
                            }
                        },
                        CeilType::Bonus => {
                            if is_word && field_draw.is_bonus(j,i) == true {
                                draw_ceil(
                                    j,
                                    i,
                                    bonus_color,
                                    enums::Color::rgb_color(
                                        230,
                                        engine::animation::ColorGenerator::get_color_component(
                                            offset as u32,
                                            4000,
                                            68,
                                            160
                                        ),
                                        230
                                    ),
                                    '!'
                                );
                            } else {
                                draw_empty_ceil(j,i, enums::Color::rgb_color(0,0,0));
                            }
                        }
                    }

                } else {
                    let alpha = (checked_value.abs() as u8) / 5;
                    let bg = enums::Color::rgb_color(63+alpha,65+alpha,82+alpha);
                    let fg = enums::Color::rgb_color(163+alpha,165+alpha,182+alpha);
                    draw_ceil(j, i, bg, fg, letter);

                    if checked_value == -1 {
                        field_draw.set(j, i, Ceil {
                            checked: 0,
                            letter: ' ',
                            ceil_type: CeilType::Empty
                        });
                        field_draw.down(j,i);
                    } else {
                        field_draw.set(j, i, Ceil {
                            checked: checked_value + 14,
                            letter: field_draw.get(j, i).letter,
                            ceil_type: CeilType::Empty
                        });
                    }
                }
            }
        }
    });

    wind.end();
    wind.show();

    app::add_idle3(move |_| {
        app::sleep(0.01);
        wind.redraw();
    });

    app.run().unwrap();

    Ok(())
}