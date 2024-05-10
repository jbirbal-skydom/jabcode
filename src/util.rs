// jabcode/src/util.rs

use std::error::Error;
use std::fmt;

// Define constants
pub const MAX_SYMBOL_NUMBER: usize = 61;
pub const MAX_SYMBOL_ROWS: usize = 3;
pub const MAX_SYMBOL_COLUMNS: usize = 3;
pub const MAX_MODULES: usize = 145;

// Define error types
#[derive(Debug)]
pub enum JABCodeError {
    InvalidInput(String),
    MemoryAllocationFailed(String),
    // Add more error variants as needed
}

impl fmt::Display for JABCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JABCodeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            JABCodeError::MemoryAllocationFailed(msg) => write!(f, "Memory allocation failed: {}", msg),
            // Add more error variants and their display implementations
        }
    }
}

impl Error for JABCodeError {}

// Define common data structures
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Implement utility functions
pub fn report_error(message: &str) {
    // Print or log the error message
    eprintln!("JABCode Error: {}", message);
}

pub fn allocate_memory<T>(size: usize) -> Result<Vec<T>, JABCodeError> {
    // Allocate memory for a vector of size `size`
    let mut vec = Vec::with_capacity(size);
    if vec.capacity() != size {
        return Err(JABCodeError::MemoryAllocationFailed(format!(
            "Failed to allocate memory for size {}",
            size
        )));
    }
    Ok(vec)
}