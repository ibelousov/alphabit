use std::borrow::BorrowMut;
use rand::Rng;
use std::cell::RefCell;
use std::ops::Index;
use std::fs;
use serde::{Serialize,Deserialize, Serializer};
use serde::ser::{SerializeStruct};

pub enum Direction {
    LEFT_TO_RIGHT,
    RIGHT_TO_LEFT,
    UP_TO_DOWN,
    DOWN_TO_UP,
    UP_LEFT_TO_DOWN_RIGHT,
    DOWN_LEFT_TO_UP_RIGHT,
    UP_RIGHT_TO_DOWN_LEFT,
    DOWN_RIGHT_TO_UP_LEFT,
    NONE
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

#[derive(Debug,Serialize, Deserialize, Clone)]
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

        for y in (0..height) {
            let mut nvec: Vec<Ceil> = Vec::new();

            for x in (0..width) {
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
        let string = String::from("АААААААААААААААААААААБББББВВВВВВВВВВВВВВВВГГГГГГДДДДДДДДДДЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЕЁЖЖЖЗЗЗЗЗИИИИИИИИИИИИИИИИИИИИИИИИИИИЙЙЙЙККККККККККККЛЛЛЛЛЛЛЛЛЛЛЛЛЛМММММММММММНННННННННННННННННННННННОООООООООООООООООООООООООООООООООООООППППППППППРРРРРРРРРРРРРРРРРСССССССССССССССССССТТТТТТТТТТТТТТТТТТТТТТУУУУУФФХХХЦЦЧЧЧЧЧШШЩЪЫЫЫЫЫЫЫЬЬЬЬЬЬЭЮЮЯЯЯЯЯЯЯ");

        for j in (0..self.height) {
            let mut random_type = rand::thread_rng().gen_range(-self.width, self.width);
            for i in (0..self.width) {
                let mut random_index = rand::thread_rng().gen_range(0, string.chars().enumerate().count());

                for (sz, ch) in string.chars().enumerate() {
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
            // print!("\n");
        }
    }

    pub fn is_blocked(&self) -> bool {
        for i in (0..self.height) {
            for j in (0..self.width) {
                if self.get(j,i).checked < 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn print(&self) {
        for j in (0..self.height) {
            for i in (0..self.width) {
                let letter = self.data.borrow()[j as usize][i as usize].letter;
                let checked = self.data.borrow()[j as usize][i as usize].checked;

                if checked != 0 {
                    print!("{}({}) ", letter, 'x');
                } else {
                    print!("{}({}) ", letter, '_');
                }
            }
            print!("{}", "\n");
        }
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

        for i in (0..self.height) {
            for j in (0..self.width) {
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

    pub fn deselect(&self, x: i32, y: i32) {
        let mut data = self.data.borrow_mut();

        for i in (0..self.height) {
            for j in (0..self.width) {
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

        let (max_val,max_x,max_y) = self.find_max();

        let word = self.get_word();
        let word2 = word.clone();
        let word3 = word.clone();

        let mut data = self.data.borrow_mut();

        let is_checked = data[y as usize][x as usize].checked == 0;
        let is_exists = max_val > 0;
        let is_previous = (data[y as usize][x as usize].checked == max_val - 1) && data[y as usize][x as usize].checked > 0;
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

            for y in (0..self.height) {
                for x in (0..self.width) {
                    if data[y as usize][x as usize].checked > 0 {
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

            let scores = self.scores.take() + ((word2.capacity() / 2) * (word2.capacity() / 2)) as i32;
            self.set_scores(scores);
        }

        drop(data);

        let word = self.get_word();
        *self.is_word_ready.borrow_mut() = self.check_word(word.to_lowercase());
    }

    pub fn get_at_value(&self, val: i32) -> char {
        for i in (0..self.height) {
            for j in (0..self.width) {
                if self.get(j,i).checked == val {
                    return self.get(j,i).letter;
                }
            }
        }

        'x'
    }

    pub fn get_word(&self) -> String {
        let mut value = String::from("");
        let (max_val,max_x, max_y) = self.find_max();

        for i in (1..max_val+1) {
            value.push(self.get_at_value(i));
        }

        value
    }

    pub fn get_direction(&self, x: i32, y: i32) -> Direction {

        let v = self.data.borrow()[y as usize][x as usize].checked;

        if v > 0 {
            if self.is_checked(x+1,y) == v + 1 {
                return Direction::LEFT_TO_RIGHT;
            } else if self.is_checked(x-1,y) == v + 1 {
                return Direction::RIGHT_TO_LEFT;
            } else if self.is_checked(x,y-1) == v + 1 {
                return Direction::DOWN_TO_UP;
            } else if self.is_checked(x,y+1) == v + 1 {
                return Direction::UP_TO_DOWN;
            } else if self.is_checked(x-1,y+1) == v + 1 {
                return Direction::UP_RIGHT_TO_DOWN_LEFT;
            } else if self.is_checked(x-1,y-1) == v + 1 {
                return Direction::DOWN_RIGHT_TO_UP_LEFT;
            } else if self.is_checked(x+1,y+1) == v + 1 {
                return Direction::UP_LEFT_TO_DOWN_RIGHT;
            } else if self.is_checked(x+1,y-1) == v + 1 {
                return Direction::DOWN_LEFT_TO_UP_RIGHT;
            }
        }

        Direction::NONE
    }

    pub fn get_before_direction(&self, x: i32, y: i32) -> Direction {
        let v = self.data.borrow()[y as usize][x as usize].checked;

        if v > 1 {
            if self.is_checked(x+1,y) == v - 1 {
                return Direction::LEFT_TO_RIGHT;
            } else if self.is_checked(x-1,y) == v - 1 {
                return Direction::RIGHT_TO_LEFT;
            } else if self.is_checked(x,y-1) == v - 1 {
                return Direction::DOWN_TO_UP;
            } else if self.is_checked(x,y+1) == v - 1 {
                return Direction::UP_TO_DOWN;
            } else if self.is_checked(x-1,y+1) == v - 1 {
                return Direction::UP_RIGHT_TO_DOWN_LEFT;
            } else if self.is_checked(x-1,y-1) == v - 1 {
                return Direction::DOWN_RIGHT_TO_UP_LEFT;
            } else if self.is_checked(x+1,y+1) == v - 1 {
                return Direction::UP_LEFT_TO_DOWN_RIGHT;
            } else if self.is_checked(x+1,y-1) == v - 1 {
                return Direction::DOWN_LEFT_TO_UP_RIGHT;
            }
        }

        Direction::NONE
    }
}