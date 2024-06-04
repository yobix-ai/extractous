
pub mod utils;
pub mod errors;

mod extract {
    pub mod auto;
    pub mod tika;
    pub mod csv;
}

pub use extract::tika;
pub use extract::csv;
pub use extract::auto::extract;

pub mod documents {
    pub mod base;
    pub mod elements;
    pub mod coordinates;
}