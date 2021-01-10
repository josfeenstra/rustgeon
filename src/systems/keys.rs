use super::console;



#[derive(Hash, PartialEq, Eq, Copy)]
pub enum Key {
    A, B, C, D, E, F,  G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Up, Down, Left, Right, Space, Control, Alt, Shift, Enter, Esc,
    N1, N2, N3, N4,  N5,  N6, N7, N8,  N9, N0, Plus, Minus, Backspace
}



impl Key {
    pub fn to_mapping(&self) -> u8 {
        match self {
            Key::A => {1}
            Key::B => {2}
            Key::C => {3}
            Key::D => {4}
            Key::E => {5}
            Key::F => {6}
            Key::G => {7}
            Key::H => {8}
            Key::I => {9}
            Key::J => {10}
            Key::K => {11}
            Key::L => {12}
            Key::M => {13}
            Key::N => {14}
            Key::O => {15}
            Key::P => {16}
            Key::Q => {17}
            Key::R => {18}
            Key::S => {19}
            Key::T => {20}
            Key::U => {21}
            Key::V => {22}
            Key::W => {23}
            Key::X => {24}
            Key::Y => {25}
            Key::Z => {26}

            Key::Up => {27}
            Key::Down => {28}
            Key::Left => {29}
            Key::Right => {30}
            Key::Space => {31}
            Key::Control => {32}
            Key::Alt => {33}
            Key::Shift => {34}
            Key::Enter => {35}
            Key::Esc => {36}

            Key::N1 => {37}
            Key::N2 => {38}
            Key::N3 => {39}
            Key::N4 => {40}
            Key::N5 => {41}
            Key::N6 => {42}
            Key::N7 => {43}
            Key::N8 => {44}
            Key::N9 => {45}
            Key::N0 => {46}

            Key::Minus => {47}
            Key::Plus => {48}
            Key::Backspace => {49}
        }
    } 

    pub fn from_string(string: &String) -> Option<Key> {
        match string.as_str() {
            "A" | "a" => Some(Key::A),
            "B" | "b" => Some(Key::B),
            "C" | "c" => Some(Key::C),
            "D" | "d" => Some(Key::D),
            "E" | "e" => Some(Key::E),
            "F" | "f" => Some(Key::F),
            "G" | "g" => Some(Key::G),
            "H" | "h" => Some(Key::H),
            "I" | "i" => Some(Key::I),
            "J" | "j" => Some(Key::J),
            "K" | "k" => Some(Key::K),
            "L" | "l" => Some(Key::L),
            "M" | "m" => Some(Key::M),
            "N" | "n" => Some(Key::N),
            "O" | "o" => Some(Key::O),
            "P" | "p" => Some(Key::P),
            "Q" | "q" => Some(Key::Q),
            "R" | "r" => Some(Key::R),
            "S" | "s" => Some(Key::S),
            "T" | "t" => Some(Key::T),
            "U" | "u" => Some(Key::U),
            "V" | "v" => Some(Key::V),
            "W" | "w" => Some(Key::W),
            "X" | "x" => Some(Key::X),
            "Y" | "y" => Some(Key::Y),
            "Z" | "z" => Some(Key::Z),

            "Up" => Some(Key::Up),
            "Down" => Some(Key::Down),
            "Left" => Some(Key::Left),
            "Right" => Some(Key::Right),
            

            " " => Some(Key::Space),
            "Control" => Some(Key::Control),
            "Alt" => Some(Key::Alt),
            "Shift" => Some(Key::Shift),
            "Enter" => Some(Key::Enter),
            "Esc" => Some(Key::Esc),

            "1" => Some(Key::N1),
            "2" => Some(Key::N2),
            "3" => Some(Key::N3),
            "4" => Some(Key::N4),
            "5" => Some(Key::N5),
            "6" => Some(Key::N6),
            "7" => Some(Key::N7),
            "8" => Some(Key::N8),
            "9" => Some(Key::N9),
            "0" => Some(Key::N0),

            "-" | "_" => Some(Key::Minus),
            "=" | "+" => Some(Key::Plus),

            _ => None
        }
    }
}

impl Clone for Key {
    fn clone(&self) -> Self {
        Key::A
    }
}