use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct Calc {
    left: f32,
    right: Option<f32>,
    symbol: String,
    is_evaluated: bool,
}

impl Calc {
    pub fn new() -> Self {
        Calc {
            left: 0.0,
            right: Option::None,
            symbol: "".to_string(),
            is_evaluated: false,
        }
    }
    pub fn add(&mut self) {
        self.left += self.right.unwrap_or(0.0);
        self.right = Option::None;
        self.symbol = "".to_string();
        self.is_evaluated = true;
    }
    pub fn sub(&mut self) {
        self.left -= self.right.unwrap_or(0.0);
        self.right = Option::None;
        self.symbol = "".to_string();
        self.is_evaluated = true;
    }
    pub fn mult(&mut self) {
        self.left *= self.right.unwrap_or(1.0);
        self.right = Option::None;
        self.symbol = "".to_string();
        self.is_evaluated = true;
    }
    pub fn div(&mut self) {
        self.left /= self.right.unwrap_or(1.0);
        self.right = Option::None;
        self.symbol = "".to_string();
        self.is_evaluated = true;
    }
    pub fn display(&self) -> String {
        if let Some(right) = &self.right {
            format!("{} {} {}", &self.left, &self.symbol, right)
        } else {
            format!("{} {}", &self.left, &self.symbol)
        }
    }
    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }
    pub fn add_display(&mut self, val: f32) {
        if self.is_evaluated
            && (self.symbol != "+"
                && self.symbol != "-"
                && self.symbol != "/"
                && self.symbol != "*")
        {
            self.left = 0.0;
            self.right = None;
            self.is_evaluated = false;
        }

        if !self.symbol.is_empty() {
            let new_val = format!("{}{}", self.right.unwrap_or(0.0), val);
            self.right = Some(new_val.parse::<f32>().unwrap());
        } else {
            let new_val = format!("{}{}", self.left, val);
            self.left = new_val.parse::<f32>().unwrap();
        }
    }
    pub fn add_symbol(&mut self, val: String) {
        self.symbol = val;
    }
    pub fn reset(&mut self) {
        self.left = 0.0;
        self.right = None;
        self.symbol = "".to_string();
    }
}
