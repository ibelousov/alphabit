mod engine;

use std::error::Error;
use std::borrow::{Borrow, BorrowMut};
use engine::*;
use fltk::{app, button::Button, frame::Frame, prelude::*, *, draw::*};
use fltk::window::Window;
use fltk::enums::{Color, Event, FrameType};
use std::sync::{mpsc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use csv::ByteRecord;
use fltk::app::MouseButton;
use fltk::image::Image;

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

fn get_green() -> i32 {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    let millis: i32 = (millis % 1000) as i32;
    let millis =  (millis - 500).abs();

    68 + (millis / 10)
}

fn draw_direction(j:i32, i:i32, direction:Direction)
{
    set_line_style(LineStyle::Solid, 3);

    match direction {
        Direction::LEFT_TO_RIGHT => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, (j + 1) * 40, i * 40 + 20 + OFFSET_Y);
        },
        Direction::RIGHT_TO_LEFT => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, j * 40, i * 40 + 20 + OFFSET_Y);
        },
        Direction::UP_TO_DOWN => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, j * 40 + 20, (i + 1) * 40 + OFFSET_Y);
        },
        Direction::DOWN_TO_UP => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y,j * 40 + 20, i * 40 + OFFSET_Y);
        },
        Direction::DOWN_RIGHT_TO_UP_LEFT => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, j * 40, i * 40 + OFFSET_Y);
        },
        Direction::UP_RIGHT_TO_DOWN_LEFT => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y,j * 40, (i+1) * 40 + OFFSET_Y);
        },
        Direction::DOWN_LEFT_TO_UP_RIGHT => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y, (j+1) * 40, i * 40 + OFFSET_Y);
        },
        Direction::UP_LEFT_TO_DOWN_RIGHT => {
            draw_line(j * 40 + 20, i * 40 + 20 + OFFSET_Y,(j + 1) * 40, (i + 1) * 40 + OFFSET_Y);
        },
        Direction::NONE => {}
    }
}

fn main() -> Result<(), confy::ConfyError> {

    let mut config: Config = confy::load(SETTINGS_NAME, None)?;

    let app = app::App::default();

    let mut field = Rc::new(config.field.clone());
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

                        confy::store(SETTINGS_NAME, None, &config);

                        f.redraw();
                    }

                    if y < OFFSET_Y {
                        return true;
                    }

                    let (cell_x, cell_y) = (x / 40, (y - OFFSET_Y) / 40);

                    if app::event_mouse_button() == MouseButton::Right {
                        field.deselect(cell_x, cell_y);
                    } else {
                        field.try_check(cell_x, cell_y);
                    }

                    config.field = (*field).clone();
                    confy::store(SETTINGS_NAME, None, &config);

                    f.redraw();

                    true
                },
                Event::NoEvent => {

                    if config.borrow_mut().position_x != f.x() ||
                        config.borrow_mut().position_x != f.y() {
                        config.borrow_mut().position_x = f.x();
                        config.borrow_mut().position_y = f.y();
                        confy::store(SETTINGS_NAME, None, &config);
                    }

                    true
                },
                _ => false
            }
        }
    });

    wind.draw(move |w| {

        set_font(enums::Font::Courier, 16);
        draw_rect_fill( 0,0,40 * WIDTH, OFFSET_Y,enums::Color::rgb_color(118,50, 40));
        set_draw_color(enums::Color::rgb_color(255,255,255));
        draw_text( &format!("ОЧКИ: {:0>5}", field_draw.get_scores()), 15, 25);

        draw_rect_fill(150,0,300, OFFSET_Y, enums::Color::rgb_color(0,140,210));
        set_draw_color(enums::Color::rgb_color(255,255,255));
        if field_draw.get_longest_word().len() > 0 {
            draw_text( &format!("{:^25}", field_draw.get_longest_word().to_uppercase()), 175, 25);
        } else {
            draw_text( &format!("{:^25}", "-"), 175, 25);
        }

        draw_rect_fill(450,0,150, OFFSET_Y, enums::Color::rgb_color(50,200,160));
        set_draw_color(enums::Color::rgb_color(255,255,255));
        draw_text( &format!("{}", "ФИНИШ"), 500, 25);
        let green = get_green() as u8;
        let color = if field_draw.is_word() == true {
            enums::Color::rgb_color(100, green,100)
        } else {
            enums::Color::rgb_color(100, 100,100)
        };

        draw_rect_fill(0, OFFSET_Y, WIDTH * 40, HEIGHT * 40, enums::Color::rgb_color(40,42,54));

        for i in (0..HEIGHT) {
            for j in (0..WIDTH) {
                let checked_value = field_draw.is_checked(j, i);

                if checked_value > 0 {

                    draw_rect_fill(j * 40 + 1, i * 40 + 1 + OFFSET_Y, 38, 38, color);
                    set_draw_color(enums::Color::White);
                    draw_text( &format!("{}", field_draw.get(j,i).letter), j * 40 + 15, i * 40 + 25 + OFFSET_Y);

                    set_draw_color(enums::Color::White);
                    draw_direction(j,i,field_draw.get_before_direction(j,i));
                    draw_direction(j,i,field_draw.get_direction(j,i));

                } else if checked_value == 0 {
                    match field_draw.get(j,i).ceil_type {
                        CeilType::Active => {
                            draw_rect_fill(j * 40 + 1, i * 40 + 1 + OFFSET_Y, 38, 38, enums::Color::rgb_color(63,65,82));
                            set_draw_color(enums::Color::rgb_color(230,230,230));
                            draw_text( &format!("{}", field_draw.get(j,i).letter), j * 40 + 15, i * 40 + 25 + OFFSET_Y);
                        },
                        CeilType::Empty => {
                            draw_rect_fill(j * 40 + 1, i * 40 + 1 + OFFSET_Y, 38, 38, enums::Color::rgb_color(63,65,82));
                        },
                        CeilType::Bonus => {
                            draw_rect_fill(j * 40 + 1, i * 40 + 1 + OFFSET_Y, 38, 38, enums::Color::rgb_color(0,0,0));
                        }
                    }

                } else {
                    let alpha = (checked_value.abs() as u8) / 5;
                    draw_rect_fill(j * 40 + 1, i * 40 + 1 + OFFSET_Y, 38, 38, color);
                    draw_rect_fill(j * 40 + 1, i * 40 + 1 + OFFSET_Y, 38, 38, enums::Color::rgb_color(63+alpha,65+alpha,82+alpha));
                    set_draw_color(enums::Color::rgb_color(163+alpha,165+alpha,182+alpha));

                    if(checked_value == -1) {
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

                    draw_text( &format!("{}", field_draw.get(j,i).letter), j * 40 + 15, i * 40 + 25 + OFFSET_Y);

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