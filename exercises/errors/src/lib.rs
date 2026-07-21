use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum DolphinError {
    #[error("The dolphin is hungry")]
    Hungry,
    #[error("The dolphin is too young")]
    TooYoung,
    #[error("The dolphin's name is too long and annoying to say")]
    LongName,
}

pub struct Dolphin {
    pub name: String,
    pub age: u8,
    pub hungry: bool,
}

impl Dolphin {
    pub fn say_your_name(&self) -> Result<String, DolphinError> {
        if self.name.len() > 10 {
            Err(DolphinError::LongName)
        } else {
            Ok(format!("Hi, my name is {} and I'm a Dolphin!", self.name))
        }
    }
    pub fn flip(&self) -> Result<String, DolphinError> {
        if self.age < 4 {
            Err(DolphinError::TooYoung)
        } else {
            Ok("Yippee, I'm doing a flip!".to_string())
        }
    }
    pub fn shake_hands(&self) -> Result<String, DolphinError> {
        if self.hungry {
            Err(DolphinError::Hungry)
        } else {
            Ok("Nice to meet you, let's shake hands!".to_string())
        }
    }
}
