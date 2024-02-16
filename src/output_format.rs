
pub struct OutputFormat {
    pub name: String,
    pub size: u32,
    pub format: String,
}

impl OutputFormat {
    pub fn new(name: String, size: u32, format: String) -> Self {
        Self { name, size, format }
    }

    pub fn app_store_outputs() -> Vec<OutputFormat> {
        let format: String = "png".to_string();
        vec![
            Self::new("1024".to_string(), 1024, format.clone()),
            Self::new("512@2x".to_string(), 1024, format.clone()),
            Self::new("512".to_string(), 512, format.clone()),
            Self::new("256@2x".to_string(), 512, format.clone()),
            Self::new("256".to_string(), 256, format.clone()),
            Self::new("128@2x".to_string(), 256, format.clone()),
            Self::new("128".to_string(), 128, format.clone()),
            Self::new("32@2x".to_string(), 64, format.clone()),
            Self::new("32".to_string(), 32, format.clone()),
            Self::new("16@2x".to_string(), 32, format.clone()),
            Self::new("16".to_string(), 16, format.clone()),
        ]
    }
}
