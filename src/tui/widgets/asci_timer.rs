use std::collections::HashMap;

use termint::{
    enums::Wrap,
    widgets::{Element, Grad},
};

#[derive(Debug, Clone, PartialEq)]
pub struct AsciTimer {
    digits: HashMap<char, Vec<&'static str>>,
    pub height: usize,
}

impl AsciTimer {
    pub fn new(
        digits: HashMap<char, Vec<&'static str>>,
        height: usize,
    ) -> Self {
        Self { digits, height }
    }

    pub fn regular() -> Self {
        let (digits, height) = get_regular();
        Self { digits, height }
    }

    pub fn element(&self, time: String) -> (Element, usize) {
        let mut lines = vec![String::new(); self.height];
        for digit in time.chars() {
            let Some(asci) = self.digits.get(&digit) else {
                continue;
            };
            for i in 0..self.height {
                lines[i].push_str(asci[i]);
            }
        }
        (
            Grad::new(lines.join("\n"), (0, 220, 255), (160, 100, 255))
                .wrap(Wrap::Letter)
                .into(),
            lines[0].chars().count(),
        )
    }
}

fn get_regular() -> (HashMap<char, Vec<&'static str>>, usize) {
    (
        HashMap::from([
            (
                '0',
                vec![
                    r"  ██████ ",
                    r" ██  ████",
                    r" ██ ██ ██",
                    r" ████  ██",
                    r"  ██████ ",
                ],
            ),
            (
                '1',
                vec![
                    r"    ██   ",
                    r"   ███   ",
                    r"    ██   ",
                    r"    ██   ",
                    r"    ██   ",
                ],
            ),
            (
                '2',
                vec![
                    r" ██████  ",
                    r"      ██ ",
                    r"  █████  ",
                    r" ██      ",
                    r" ███████ ",
                ],
            ),
            (
                '3',
                vec![
                    r" ██████  ",
                    r"      ██ ",
                    r"  █████  ",
                    r"      ██ ",
                    r" ██████  ",
                ],
            ),
            (
                '4',
                vec![
                    r" ██   ██ ",
                    r" ██   ██ ",
                    r" ███████ ",
                    r"      ██ ",
                    r"      ██ ",
                ],
            ),
            (
                '5',
                vec![
                    r" ███████ ",
                    r" ██      ",
                    r" ███████ ",
                    r"      ██ ",
                    r" ███████ ",
                ],
            ),
            (
                '6',
                vec![
                    r"  ██████ ",
                    r" ██      ",
                    r" ███████ ",
                    r" ██    ██",
                    r"  ██████ ",
                ],
            ),
            (
                '7',
                vec![
                    r" ███████ ",
                    r"      ██ ",
                    r"     ██  ",
                    r"    ██   ",
                    r"    ██   ",
                ],
            ),
            (
                '8',
                vec![
                    r"  █████  ",
                    r" ██   ██ ",
                    r"  █████  ",
                    r" ██   ██ ",
                    r"  █████  ",
                ],
            ),
            (
                '9',
                vec![
                    r"  █████  ",
                    r" ██   ██ ",
                    r"  ██████ ",
                    r"      ██ ",
                    r"  █████  ",
                ],
            ),
            (':', vec![r"    ", r" ██ ", r"    ", r" ██ ", r"    "]),
        ]),
        5,
    )
}
