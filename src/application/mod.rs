pub(crate) mod animation;
pub(crate) mod settings;
pub(crate) mod ui;
pub(crate) mod field;
pub(crate) mod sound;

use std::cell::RefCell;
use fltk::{app, prelude::*, *, window::DoubleWindow};
use fltk::window::Window;
use fltk::enums::{Event};
use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::app::MouseButton;
use settings::*;
use ui::*;
use field::*;
use animation::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    field: Field,
    position_x: i32,
    position_y: i32,
    table_of_leaders: Vec<(i32, String)>
}

impl ::std::default::Default for Config {
    fn default() -> Self {

        let field = Field::new(WIDTH, HEIGHT);

        field.generate();

        Self {
            field: field,
            position_x: 0,
            position_y: 0,
            table_of_leaders: vec![
                (0, String::from("-")),
                (0, String::from("-")),
                (0, String::from("-")),
                (0, String::from("-")),
                (0, String::from("-")),
                (0, String::from("-")),
                (0, String::from("-")),
                (0, String::from("-")),
                (0, String::from("-")),
                (0, String::from("-")),
            ]
        }
    }
}

fn load_icon(wind: &mut DoubleWindow) {
    let image = image::PngImage::from_data(
        include_bytes!("../../assets/icon.png")
    ).unwrap();

    wind.set_icon(Some(image));
}

pub fn app() -> Result<(), confy::ConfyError> {

    let mut config: Config = confy::load(SETTINGS_NAME, None)?;

    let app = app::App::default();

    let field = Rc::new(config.field.clone());
    let field_draw = Rc::clone(&field);
    let table_of_leaders = Rc::new(RefCell::new(config.table_of_leaders.clone()));
    let table_of_leaders_draw = Rc::clone(&table_of_leaders);

    let mut wind = Window::new(
        config.position_x,
        config.position_y,
        WIDTH * CELL_SIZE + SIDEBAR_WIDTH,
        HEIGHT * CELL_SIZE + OFFSET_Y,
        TITLE
    );

    load_icon(&mut wind);

    wind.handle({
        move |f, ev| {
            match ev {
                Event::Push => {

                    if field.is_blocked() {
                        return true;
                    }

                    let (x,y) = app::event_coords();

                    if y > 5 && y < OFFSET_Y - 10 && x > 455 && x < 600 {

                        let mut leaders = (*table_of_leaders).borrow_mut();
                        for (idx, leader) in leaders.iter().enumerate() {
                            if leader.0 < field.get_scores() {
                                leaders.insert(
                                    idx,
                                    (field.get_scores(), field.get_longest_word()
                                    )
                                );
                                break;
                            }
                        }

                        if leaders.len() > 10 {
                            leaders.pop();
                        }

                        field.generate();
                        field.set_longest_word(String::new());
                        field.set_scores(0);

                        config.field = (*field).clone();
                        config.table_of_leaders = leaders.clone();

                        confy::store(SETTINGS_NAME, None, &config)
                            .expect(CANNOT_SAVE_MSG);

                        f.redraw();
                    }

                    if y < OFFSET_Y {
                        return true;
                    }

                    let (cell_x, cell_y) = (x / CELL_SIZE, (y - OFFSET_Y) / CELL_SIZE);

                    if app::event_mouse_button() == MouseButton::Right {
                        field.deselect();
                    } else {
                        field.try_check(cell_x, cell_y);
                        sound::play_sound();
                    }

                    config.field = (*field).clone();
                    confy::store(SETTINGS_NAME, None, &config)
                        .expect(CANNOT_SAVE_MSG);

                    f.redraw();

                    true
                },
                Event::NoEvent => {

                    if config.position_x != f.x() || config.position_x != f.y() {
                        config.position_x = f.x();
                        config.position_y = f.y();
                        confy::store(SETTINGS_NAME, None, &config)
                            .expect(CANNOT_SAVE_MSG);
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

        draw_leaders_table((*table_of_leaders_draw).borrow());

        draw_controls();

        let is_word = field_draw.is_word();
        let is_bonus_exists = field_draw.is_bonus_exists();

        draw_bg();

        for i in 0..field_draw.get_height() {
            for j in 0..field_draw.get_width() {
                let offset = (j + i * field_draw.get_width()) * 225;
                let color = if is_word {
                    enums::Color::rgb_color(
                        100,
                        ColorGenerator::get_color_component(
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
                    ColorGenerator::get_color_component(
                        offset as u32,
                        2000,
                        68,
                        160
                    ),
                    ColorGenerator::get_color_component(
                        offset as u32,
                        3222,
                        68,
                        160
                    ),
                    ColorGenerator::get_color_component(
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
                                        ColorGenerator::get_color_component(
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