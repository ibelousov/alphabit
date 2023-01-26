use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use crate::application::settings;

pub enum Direction {
    LeftToRight,
    RightToLeft,
    UpToDown,
    DownToUp,
    UpLeftToDownRight,
    DownLeftToUpRight,
    UpRightToDownLeft,
    DownRightToUpLeft,
    None
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum CeilType {
    Active,
    Empty,
    Bonus
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Ceil {
    pub letter: char,
    pub checked: i32,
    pub ceil_type: CeilType
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    pub scores: RefCell<i32>,
    is_word_ready: RefCell<bool>,
    most_lengthy_word: RefCell<String>,
    height: i32,
    width: i32,
    data: RefCell<Vec<Vec<Ceil>>>
}

impl  Field {
    pub fn new(width: i32, height: i32) -> Self {
        let data: RefCell<Vec<Vec<Ceil>>> = RefCell::new(Vec::new());

        for _y in 0..height {
            let mut nvec: Vec<Ceil> = Vec::new();

            for _x in 0..width {
                nvec.push(Ceil {
                    letter: ' ',
                    checked: 0,
                    ceil_type: CeilType::Active
                })
            }

            data.borrow_mut().push(nvec);
        }

        Field {
            scores: RefCell::new(0),
            is_word_ready: RefCell::new(false),
            width,
            height,
            data,
            most_lengthy_word: RefCell::new(String::new())
        }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_scores(&self) -> i32 {
        *self.scores.borrow()
    }

    pub fn get_longest_word(&self) -> String {
        self.most_lengthy_word.borrow().clone()
    }

    pub fn set_longest_word(&self, word: String) {
        *self.most_lengthy_word.borrow_mut() = word;
    }

    pub fn set_scores(&self, scores: i32) {
        let mut scorestochange = self.scores.borrow_mut();
        *scorestochange = scores;
    }

    pub fn generate(&self) {
        let mut letter_frequency = HashMap::new();
        letter_frequency.insert('А', 8.01);
        letter_frequency.insert('Б', 1.59);
        letter_frequency.insert('В', 4.54);
        letter_frequency.insert('Г', 1.70);
        letter_frequency.insert('Д', 2.98);
        letter_frequency.insert('Е',8.45);
        letter_frequency.insert('Ё', 0.04);
        letter_frequency.insert('Ж',0.94);
        letter_frequency.insert('З',1.65);
        letter_frequency.insert('И',7.35);
        letter_frequency.insert('Й',1.21);
        letter_frequency.insert('К',3.49);
        letter_frequency.insert('Л',4.40);
        letter_frequency.insert('М',3.21);
        letter_frequency.insert('Н',6.70);
        letter_frequency.insert('О',10.97);
        letter_frequency.insert('П',2.81);
        letter_frequency.insert('Р',4.73);
        letter_frequency.insert('С',5.47);
        letter_frequency.insert('Т',6.26);
        letter_frequency.insert('У',2.62);
        letter_frequency.insert('Ф',0.26);
        letter_frequency.insert('Х',0.97);
        letter_frequency.insert('Ц',0.48);
        letter_frequency.insert('Ч',1.44);
        letter_frequency.insert('Ш', 0.73);
        letter_frequency.insert('Щ',0.36);
        letter_frequency.insert('Ъ',0.04);
        letter_frequency.insert('Ы',1.90);
        letter_frequency.insert('Ь',1.74);
        letter_frequency.insert('Э',0.32);
        letter_frequency.insert('Ю',0.64);
        letter_frequency.insert('Я',2.01);

        let mut string = String::new();

        for (letter, percent) in &letter_frequency {
            let count = (percent * 100.0) as usize;

            string.push_str(String::from(*letter).repeat(count).as_str());
        }

        for j in 0..self.height {
            let random_type = rand::thread_rng().gen_range(-self.width, self.width);
            for i in 0..self.width {
                let mut random_index = rand::thread_rng().gen_range(0, string.chars().enumerate().count());

                for ch in string.chars() {
                    if random_index == 0 {

                        let ceil_type = if (random_type as i32) == i {
                            CeilType::Bonus
                        } else {
                            CeilType::Active
                        };

                        self.data.borrow_mut()[j as usize][i as usize] = Ceil {
                            checked: 0,
                            letter: ch,
                            ceil_type: ceil_type
                        };
                        break;
                    }
                    random_index -= 1;
                }
            }
        }
    }

    pub fn is_blocked(&self) -> bool {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.get(j,i).checked < 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        return x > 0 && y > 0 && x < self.width && y < self.height;
    }

    pub fn get(&self, x: i32, y: i32) -> Ceil {
        self.data.borrow()[y as usize][x as usize]
    }

    pub fn set(&self, x: i32, y: i32, ceil: Ceil) {
        self.data.borrow_mut()[y as usize][x as usize] = ceil;
    }

    pub fn is_word(&self) -> bool {
        *self.is_word_ready.borrow()
    }

    pub fn check_word(&self, word: String) -> bool {

        let dictionary = include_str!("../../assets/words.txt").split("\n");

        let word2 = word.replace("е","ё");
        let word3 = word.replace("и", "й");

        for word_from_dictionary in dictionary {

            let normalized_word = word_from_dictionary.trim().to_string().to_lowercase();

            if normalized_word.eq(&word) ||
                normalized_word.eq(&word2) ||
                normalized_word.eq(&word3) {
                return true;
            }
        }

        false
    }

    pub fn is_checked(&self, x: i32, y: i32) -> i32 {
        if x > self.width - 1 || y > self.height - 1 || x < 0 || y < 0 {
            return -1;
        }

        self.data.borrow()[y as usize][x as usize].checked
    }

    pub fn find_max(&self) -> (i32,i32,i32) {
        let mut max: i32 = 0;
        let mut max_x: i32 = -1;
        let mut max_y: i32 = -1;

        for i in 0..self.height {
            for j in 0..self.width {
                if max < self.data.borrow()[i as usize][j as usize].checked {
                    max = self.data.borrow()[i as usize][j as usize].checked;
                    max_x = j;
                    max_y = i;
                }
            }
        }

        (max,max_x,max_y)
    }

    pub fn down(&self, x: i32, mut y: i32) {
        let mut data = self.data.borrow_mut();

        let mut y_n = y - 1;
        while y_n > -1 && data[y_n as usize][x as usize].checked >= 0 {
            let temp = data[y as usize][x as usize];
            data[y as usize][x as usize] = data[y_n as usize][x as usize];
            data[y_n as usize][x as usize] = temp;
            y_n -= 1;
            y -= 1;
        }
    }

    pub fn deselect(&self) {
        let mut data = self.data.borrow_mut();

        for i in 0..self.height {
            for j in 0..self.width {
                data[i as usize][j as usize] = Ceil {
                    checked: 0,
                    letter: data[i as usize][j as usize].letter,
                    ceil_type: data[i as usize][j as usize].ceil_type
                };
            }
        }
    }

    pub fn try_check(&self, x: i32, y: i32) {

        if matches!(self.get(x,y).ceil_type, CeilType::Empty) ||
            matches!(self.get(x,y).ceil_type, CeilType::Bonus){
            return
        };

        let mut bonus_lines = vec![];

        for y in 0..self.height {
            if self.is_on_the_bonus_line(0,y) {
                bonus_lines.push(y);
            }
        }

        let (max_val,max_x,max_y) = self.find_max();

        let word = self.get_word();
        let word2 = word.clone();

        let mut data = self.data.borrow_mut();

        let is_checked = data[y as usize][x as usize].checked == 0;
        let is_exists = max_val > 0;
        let is_near = ((max_x - x).abs() == 1 || (max_y - y).abs() == 1) &&
            ((max_x - x).abs() <= 1 && (max_y - y).abs() <= 1) &&
            is_checked;
        let is_same = (x == max_x) && (y == max_y);

        if is_near {
            data[y as usize][x as usize] = Ceil {
                checked: max_val + 1,
                letter:data[y as usize][x as usize].letter,
                ceil_type: CeilType::Active
            };
        } else if !is_exists {
            data[y as usize][x as usize] = Ceil {
                checked: 1,
                letter: data[y as usize][x as usize].letter,
                ceil_type: CeilType::Active
            };
        } else if is_same && self.check_word(word.to_lowercase()) {
            let scores = self.scores.take();
            let mut scores_append = 0;

            for y in 0..self.height {
                for x in 0..self.width {
                    let is_checked = data[y as usize][x as usize].checked > 0;
                    let is_bonus_line = bonus_lines.contains(&y);
                    if is_checked || is_bonus_line {
                        scores_append += if is_checked && bonus_lines.len() > 0 {
                            settings::SCORES_FOR_BONUS_AND_CHECKED
                        } else if !is_checked && is_bonus_line {
                            settings::SCORES_FOR_BONUS_LINE
                        } else {
                            settings::SCORES_FOR_CHECKED
                        };
                        data[y as usize][x as usize] = Ceil {
                            letter: data[y as usize][x as usize].letter,
                            checked: -253,
                            ceil_type: CeilType::Empty
                        };
                    }
                }
            }

            if self.most_lengthy_word.borrow_mut().len() < word2.len() {
                *self.most_lengthy_word.borrow_mut() = word2.clone();
            }

            self.set_scores(scores + (scores_append * 2));
        }

        drop(data);

        let word = self.get_word();
        *self.is_word_ready.borrow_mut() = self.check_word(word.to_lowercase());
    }

    pub fn is_bonus(&self, x: i32, y: i32) -> bool {
        let ceil = self.get(x,y);

        let xypairs = vec![
            (-1,-1),(0,-1),(1,-1),
            (-1,0),(1,0),
            (-1,1),(0,1),(-1,1)
        ];

        match ceil.ceil_type {
            CeilType::Bonus => {
                for (offset_x,offset_y) in xypairs {
                    let nx = x + offset_x;
                    let ny = y + offset_y;
                    if self.is_valid(nx, ny) && self.get(nx, ny).checked > 0 {
                        return true
                    }
                }
            },
            _ => {}
        }

        false
    }

    pub fn is_bonus_exists(&self) -> bool {

        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_bonus(x,y) {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_on_the_bonus_line(&self, _x: i32, y: i32) -> bool {

        for nx in 0..self.width {
            if matches!(self.get(nx,y).ceil_type, CeilType::Bonus) && self.is_bonus(nx,y) {
                return true;
            }
        }

        false
    }

    pub fn get_at_value(&self, val: i32) -> char {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.get(j,i).checked == val {
                    return self.get(j,i).letter;
                }
            }
        }

        'x'
    }

    pub fn get_word(&self) -> String {
        let mut value = String::from("");
        let max = self.find_max();

        for i in 1..max.0+1 {
            value.push(self.get_at_value(i));
        }

        value
    }

    pub fn get_direction(&self, x: i32, y: i32) -> Direction {

        let v = self.data.borrow()[y as usize][x as usize].checked;

        if v > 0 {
            if self.is_checked(x+1,y) == v + 1 {
                return Direction::LeftToRight;
            } else if self.is_checked(x-1,y) == v + 1 {
                return Direction::RightToLeft;
            } else if self.is_checked(x,y-1) == v + 1 {
                return Direction::DownToUp;
            } else if self.is_checked(x,y+1) == v + 1 {
                return Direction::UpToDown;
            } else if self.is_checked(x-1,y+1) == v + 1 {
                return Direction::UpRightToDownLeft;
            } else if self.is_checked(x-1,y-1) == v + 1 {
                return Direction::DownRightToUpLeft;
            } else if self.is_checked(x+1,y+1) == v + 1 {
                return Direction::UpLeftToDownRight;
            } else if self.is_checked(x+1,y-1) == v + 1 {
                return Direction::DownLeftToUpRight;
            }
        }

        Direction::None
    }

    pub fn get_before_direction(&self, x: i32, y: i32) -> Direction {
        let v = self.data.borrow()[y as usize][x as usize].checked;

        if v > 1 {
            if self.is_checked(x+1,y) == v - 1 {
                return Direction::LeftToRight;
            } else if self.is_checked(x-1,y) == v - 1 {
                return Direction::RightToLeft;
            } else if self.is_checked(x,y-1) == v - 1 {
                return Direction::DownToUp;
            } else if self.is_checked(x,y+1) == v - 1 {
                return Direction::UpToDown;
            } else if self.is_checked(x-1,y+1) == v - 1 {
                return Direction::UpRightToDownLeft;
            } else if self.is_checked(x-1,y-1) == v - 1 {
                return Direction::DownRightToUpLeft;
            } else if self.is_checked(x+1,y+1) == v - 1 {
                return Direction::UpLeftToDownRight;
            } else if self.is_checked(x+1,y-1) == v - 1 {
                return Direction::DownLeftToUpRight;
            }
        }

        Direction::None
    }
}