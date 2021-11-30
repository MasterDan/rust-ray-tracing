struct ColorRgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl for ColorRgb {
    fn new(r:u8,g:u8,b:u8) -> Self {
        ColorRgb {
            red: r,
            green: g,
            blue: b
        }
    }
}

impl fmt::Display for ColorRgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt{
        
    }
}