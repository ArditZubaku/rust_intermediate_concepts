// Once you've got the documentation here, run `cargo doc --no-deps --open` and take a look!

//! A pumpkin is a cultivated winter squash in the genus Cucurbita.
//! The term is most commonly applied to round, orange-colored squash varieties, but does not possess a scientific definition.
//! It may be used in reference to many different squashes of varied appearance and belonging to multiple species in the Cucurbita genus.
//!
//! ![pumpkin image](https://upload.wikimedia.org/wikipedia/commons/5/5c/FrenchMarketPumpkinsB.jpg)

/// Big orange thing
///
/// # Recipes
///
/// Recipes will most assuredly be coming very, very soon!
pub struct Pumpkin {
    /// `roundness` is a percentage
    pub roundness: f32,
    /// `orangeness` is a number from 8 to 27
    pub orangeness: i32,
}

// 4. Document the "smash" method. Explain that

impl Pumpkin {
    /// If you smash the pumpkin, it will be gone. Then it can't be used for pie. 😭
    pub fn smash(self) {}
}

// 5. Document that BURNT_ORANGE is for the "orangeness" field in the Pumpkin struct.
// - Link to the Pumpkin struct in your description

/// `BURNT_ORANGE` is for the `orangeness` field in the [Pumpkin] struct.
pub const BURNT_ORANGE: i32 = 13;

/// For internal use only. In fact, this documentation is so private that it won't be generated.
/// At least not by default. But if you pass the correct option in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}
