fn main() {
    // A `derive` macro is a way to automatically generate code for your structs or enums.
    // Instead of writing tedious boilerplate by hand to implement a trait, you just tag your data structure
    // with #[derive(...)], and the compiler writes the implementation for you at compile time.
    #[derive(Debug, Clone)]
    struct Puzzle {
        num_pieces: u32,
        name: String,
    }

    let p = Puzzle {
        num_pieces: 1,
        name: String::from("Test"),
    };

    println!("{:#?}", p.num_pieces);
    println!("{:#?}", p.name);
    println!("{:?}", p);
    println!("{:#?}", p);

    let p2 = p.clone();
    println!("{:#?}", p2);

    // Copy trait will copy the data in cases when data is moved
    // it makes sense for small data that fits entirely in the stack
    // if the type uses the heap at all then it cannot impl Copy
    //
    // Always Copy: Primitive types (i32, u64, f64, bool, char).
    // Usually Copy: Small structures, like a 2D or 3D coordinate (struct Point { x: f64, y: f64 } — 16 bytes).
    //
    // A common rule of thumb in systems programming is that data structures under 64 to 128 bytes are perfectly fine to Copy.

    #[derive(Debug, Clone, Copy)]
    pub enum PuzzleType {
        Jigsaw,
    }

    fn print_puzzle(puzzle: PuzzleType) {
        println!("Reviewing puzzle: {:?}", puzzle);
    }

    let my_puzzle = PuzzleType::Jigsaw;
    // --- THE BENEFIT OF 'COPY' ---
    // Because of `Copy`, this does NOT move `my_puzzle`.
    // It implicitly makes a perfect, bitwise copy on the stack.
    print_puzzle(my_puzzle);

    // Look! `my_puzzle` is still perfectly valid here.
    // If we removed `Copy` from the derive macro, the code would FAIL to compile right here
    // with an error: "use of moved value: `my_puzzle`".
    println!("I can still use my_puzzle here: {:?}", my_puzzle);

    impl Default for Puzzle {
        fn default() -> Self {
            Self {
                num_pieces: 0,
                name: "Forest Lake".to_string(),
            }
        }
    }
    impl PartialEq for Puzzle {
        // These are all the same thing, just syntax sugar, the last being the idiomatic way
        // fn eq(self: &Self, other: &Self) -> bool {
        // fn eq(self: &Puzzle, other: &Puzzle) -> bool {
        fn eq(&self, other: &Self) -> bool {
            self.num_pieces == other.num_pieces
                && self.name.to_lowercase() == other.name.to_lowercase()
        }
    }

    let p3 = Puzzle {
        num_pieces: 3_000,
        ..Default::default()
    };

    println!("{:?}", p3);

    // If you implement `From` then `Into` is implemented automatically for you
    impl From<&Puzzle> for String {
        fn from(value: &Puzzle) -> Self {
            value.name.clone()
        }
    }

    let p4 = Puzzle::default();
    // let p5 = p4.clone();
    // let s2: &String = p5.into();
    // println!("{}", s2);

    // We can now get the String representation of the struct because Into is implemented for us
    let s = String::from(&p4);
    println!("{}", s);

    // This basically excepts any type that has a String representation
    fn show<T: Into<String>>(s: T) {
        println!("{}", s.into())
    }

    let p5 = Puzzle::default();
    show(&p5);
}
