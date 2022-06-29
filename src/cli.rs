

pub struct Cli {
    pub red: String,
    pub green: String,
    pub blue: String,
    pub reset: String
}
impl Cli {
    pub fn new()-> Self {
        Self { red: "\x1b[31m".to_string(), green: "\x1b[32m".to_string(), blue: "\x1b[34m".to_string(), reset: "\x1b[0m".to_string() }
    }
    
    pub fn ascii(&self) {
    
        println!("{}  
█▀█ █▀▄▀█ █▀▀ █▀▀ ▄▀█
█▄█ █░▀░█ ██▄ █▄█ █▀█
        {}", &self.red, &self.reset);
    }
}
