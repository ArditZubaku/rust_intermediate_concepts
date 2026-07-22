use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        debug!(target: "Frog::new", "A new Frog has been created");
        Default::default()
    }
    pub fn hop(&mut self) {
        self.energy -= 1;
        info!("Frog hopped! Energy left {}", self.energy);
        if self.energy == 0 {
            warn!("Frog will go to sleep since he ran out of energy ");
            self.sleep();
        }
    }
    pub fn sleep(&mut self) {
        if self.sleeping {
            error!("the Frog is already asleep")
        } else {
            self.sleeping = true;
        }
    }
}

impl Default for Frog {
    fn default() -> Self {
        let f = Frog {
            energy: 5,
            sleeping: false,
        };

        trace!("a default value was generated {:?}", f);

        f
    }
}
