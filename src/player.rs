use std::{fs::File, path::Path};

use raplay::{Sink, source::Symph};

use crate::error::Error;

#[derive(Debug)]
pub struct Player {
    sink: Sink,
}

impl Player {
    pub fn new() -> Self {
        Self {
            sink: Sink::default(),
        }
    }

    pub fn play(&mut self, file: &Path) -> Result<(), Error> {
        let file = File::open(file)?;
        let src = Symph::try_new(file, &Default::default())?;
        self.sink.load(Box::new(src), true)?;
        Ok(())
    }
}
