// jabcode/src/util.rs

use std::error::Error;
use std::fmt;

// Define constants
pub const MAX_SYMBOL_NUMBER: usize = 61;
pub const MAX_SYMBOL_ROWS: usize = 3;
pub const MAX_SYMBOL_COLUMNS: usize = 3;
pub const MAX_MODULES: usize = 145;
pub const MAX_ALLOWABLE_SIZE: usize = 14000;

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
    if size > MAX_ALLOWABLE_SIZE {  // Define a reasonable MAX_ALLOWABLE_SIZE
        return Err(JABCodeError::MemoryAllocationFailed("Requested size too large".to_string()));
    }
    let mut vec = Vec::with_capacity(size);
    // vec.push(Default::default());  //  Force memory allocation
    Ok(vec)
}


// types
type JabByte = u8;
type JabChar = i8;
type JabBoolean = bool;
type JabInt32 = i32;
type JabUint32 = u32;
type JabInt16 = i16;
type JabUint16 = u16;
type JabInt64 = i64;
type JabUint64 = u64;
type JabFloat = f32;
type JabDouble = f64;


//finder pattern
const 	FP0_CORE_COLOR: usize = 0;
const 	FP1_CORE_COLOR: usize = 0;
const 	FP2_CORE_COLOR: usize = 6;
const 	FP3_CORE_COLOR: usize = 3;



const LPDC_METADATA_SEED: u64 = 38545;
const LPDC_MESSAGE_SEED: u64 = 785465;

const MAX_COLOR_NUMBER: usize = 256;
const MAX_SIZE_ENCODING_MODE: usize = 256;
const JAB_ENCODING_MODES: usize = 6;
const ENC_MAX: usize = 1000000;
const NUMBER_OF_MASK_PATTERNS: usize = 8;
const DEFAULT_SYMBOL_NUMBER: usize = 1;
const DEFAULT_MODULE_SIZE: usize = 12;
const DEFAULT_COLOR_NUMBER: usize = 8;
const DEFAULT_MODULE_COLOR_MODE: usize = 2;
const DEFAULT_ECC_LEVEL: usize = 3;
const DEFAULT_MASKING_REFERENCE: usize = 7;
const DISTANCE_TO_BORDER: usize = 4;
const MAX_ALIGNMENT_NUMBER: usize = 9;
const COLOR_PALETTE_NUMBER: usize = 4;
const BITMAP_BITS_PER_PIXEL: usize = 32;
const BITMAP_BITS_PER_CHANNEL: usize = 8;
const BITMAP_CHANNEL_COUNT: usize = 4;
const JAB_SUCCESS: bool = true;
const JAB_FAILURE: bool = false;
const NORMAL_DECODE: usize = 0;
const COMPATIBLE_DECODE: usize = 1;

fn version_to_size(version: usize) -> usize {
    version * 4 + 17
}

fn size_to_version(size: usize) -> usize {
    (size - 17) / 4
}


/// 2-dimensional integer vector
#[derive(Debug, Copy, Clone)]
pub struct JabVector2d {
    pub x: JabInt32,
    pub y: JabInt32,
}

/// 2-dimensional float vector
#[derive(Debug, Copy, Clone)]
pub struct JabPoint {
    pub x: JabFloat,
    pub y: JabFloat,
}

/// Data structure
#[derive(Debug)]
pub struct JabData {
    pub length: JabInt32,
    pub data: Vec<JabChar>,
}

/// Code bitmap
#[derive(Debug)]
pub struct JabBitmap {
    pub width: JabInt32,
    pub height: JabInt32,
    pub bits_per_pixel: JabInt32,
    pub bits_per_channel: JabInt32,
    pub channel_count: JabInt32,
    pub pixel: Vec<JabByte>,
}

/// Symbol parameters
#[derive(Debug)]
pub struct JabSymbol {
    pub index: JabInt32,
    pub side_size: JabVector2d,
    pub host: JabInt32,
    pub slaves: [JabInt32; 4],
    pub wcwr: [JabInt32; 2],
    pub data: Option<Box<JabData>>,
    pub data_map: Option<Box<[JabByte]>>,
    pub metadata: Option<Box<JabData>>,
    pub matrix: Option<Box<[JabByte]>>,
}

/// Encode parameters
#[derive(Debug)]
pub struct JabEncode {
    pub color_number: JabInt32,
    pub symbol_number: JabInt32,
    pub module_size: JabInt32,
    pub master_symbol_width: JabInt32,
    pub master_symbol_height: JabInt32,
    pub palette: Option<Box<[JabByte]>>,
    pub symbol_versions: Option<Box<[JabVector2d]>>,
    pub symbol_ecc_levels: Option<Box<[JabByte]>>,
    pub symbol_positions: Option<Box<[JabInt32]>>,
    pub symbols: Option<Box<[JabSymbol]>>,
    pub bitmap: Option<Box<JabBitmap>>,
}

pub struct JabCodeData {
    pub data: Option<Box<JabData>>,
    pub filename: Option<String>,
    pub color_number: JabInt32,
    pub symbol_number: JabInt32,
    pub module_size: JabInt32,
    pub master_symbol_width: JabInt32,
    pub master_symbol_height: JabInt32,
    pub symbol_positions: Option<Box<[JabInt32]>>,
    pub symbol_positions_number: JabInt32,
    pub symbol_versions: Option<Box<[JabVector2d]>>,
    pub symbol_versions_number: JabInt32,
    pub symbol_ecc_levels: Option<Box<[JabInt32]>>,
    pub symbol_ecc_levels_number: JabInt32,
    pub color_space: JabInt32,
}


//Color profile
static JAB_DEFAULT_PALETTE: [u8; 24] = [
    0,   0,   0,   // 0: black
    0,   0, 255,   // 1: blue
    0, 255,   0,   // 2: green
    0, 255, 255,   // 3: cyan
    255, 0,   0,   // 4: red
    255, 0, 255,   // 5: magenta
    255, 255, 0,   // 6: yellow
    255, 255, 255, // 7: white
];


impl Default for JabCodeData {
    fn default() -> Self {
        JabCodeData {
            data: None,
            filename: None,
            color_number: 0,
            symbol_number: 0,
            module_size: 0,
            master_symbol_width: 0,
            master_symbol_height: 0,
            symbol_positions: None,
            symbol_positions_number: 0,
            symbol_versions: None,
            symbol_versions_number: 0,
            symbol_ecc_levels: None,
            symbol_ecc_levels_number: 0,
            color_space: 0,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_allocation() {
        let result: Result<Vec<i32>, JABCodeError> = allocate_memory(10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().capacity(), 10);
    }

    #[test]
    fn test_version_to_size() {
        assert_eq!(version_to_size(1), 21);
        assert_eq!(version_to_size(2), 25);
    }

    #[test]
    fn test_size_to_version() {
        assert_eq!(size_to_version(21), 1);
        assert_eq!(size_to_version(25), 2);
    }

    #[test]
    fn test_default_jab_code_data() {
        let default_data = JabCodeData::default();
        assert_eq!(default_data.color_number, 0);
        assert_eq!(default_data.symbol_number, 0);
    }

        #[test]
    fn test_memory_allocation_failure() {
        let result = allocate_memory::<i32>(usize::MAX);
        assert!(result.is_err());
    }


    // You can add more tests for other functions or error cases.
}