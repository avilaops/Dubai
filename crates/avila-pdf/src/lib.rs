// AvilaPdf - Native PDF Generation
// Zero External Dependencies 🦀

use std::fs::File;
use std::io::{self, Write};

pub struct PdfDocument {
    pages: Vec<PdfPage>,
    metadata: PdfMetadata,
}

pub struct PdfPage {
    content: Vec<String>,
    width: f32,
    height: f32,
}

pub struct PdfMetadata {
    pub title: String,
    pub author: String,
    pub subject: String,
    pub creator: String,
}

impl Default for PdfMetadata {
    fn default() -> Self {
        Self {
            title: String::new(),
            author: String::new(),
            subject: String::new(),
            creator: "AvilaPdf".to_string(),
        }
    }
}

impl PdfDocument {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            metadata: PdfMetadata::default(),
        }
    }

    pub fn with_metadata(mut self, metadata: PdfMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_page(&mut self, width: f32, height: f32) -> &mut PdfPage {
        self.pages.push(PdfPage {
            content: Vec::new(),
            width,
            height,
        });
        self.pages.last_mut().unwrap()
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        
        // PDF Header
        writeln!(file, "%PDF-1.4")?;
        writeln!(file, "%âãÏÓ")?; // Binary comment for PDF readers

        // Catalog
        writeln!(file, "1 0 obj")?;
        writeln!(file, "<<")?;
        writeln!(file, "/Type /Catalog")?;
        writeln!(file, "/Pages 2 0 R")?;
        writeln!(file, ">>")?;
        writeln!(file, "endobj")?;

        // Pages Object
        writeln!(file, "2 0 obj")?;
        writeln!(file, "<<")?;
        writeln!(file, "/Type /Pages")?;
        writeln!(file, "/Count {}", self.pages.len())?;
        write!(file, "/Kids [")?;
        for i in 0..self.pages.len() {
            write!(file, " {} 0 R", 3 + i)?;
        }
        writeln!(file, " ]")?;
        writeln!(file, ">>")?;
        writeln!(file, "endobj")?;

        // Page Objects
        for (i, page) in self.pages.iter().enumerate() {
            let obj_num = 3 + i;
            writeln!(file, "{} 0 obj", obj_num)?;
            writeln!(file, "<<")?;
            writeln!(file, "/Type /Page")?;
            writeln!(file, "/Parent 2 0 R")?;
            writeln!(file, "/MediaBox [0 0 {} {}]", page.width, page.height)?;
            writeln!(file, "/Contents {} 0 R", obj_num + self.pages.len())?;
            writeln!(file, ">>")?;
            writeln!(file, "endobj")?;
        }

        // Content Streams
        for (i, page) in self.pages.iter().enumerate() {
            let content = page.content.join("\n");
            let obj_num = 3 + self.pages.len() + i;
            
            writeln!(file, "{} 0 obj", obj_num)?;
            writeln!(file, "<<")?;
            writeln!(file, "/Length {}", content.len())?;
            writeln!(file, ">>")?;
            writeln!(file, "stream")?;
            write!(file, "{}", content)?;
            writeln!(file, "\nendstream")?;
            writeln!(file, "endobj")?;
        }

        // Trailer
        writeln!(file, "xref")?;
        writeln!(file, "0 {}", 3 + (self.pages.len() * 2))?;
        writeln!(file, "0000000000 65535 f ")?;
        for _ in 0..(2 + (self.pages.len() * 2)) {
            writeln!(file, "0000000000 00000 n ")?;
        }

        writeln!(file, "trailer")?;
        writeln!(file, "<<")?;
        writeln!(file, "/Size {}", 3 + (self.pages.len() * 2))?;
        writeln!(file, "/Root 1 0 R")?;
        writeln!(file, ">>")?;
        writeln!(file, "startxref")?;
        writeln!(file, "0")?;
        writeln!(file, "%%EOF")?;

        Ok(())
    }
}

impl Default for PdfDocument {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfPage {
    pub fn add_text(&mut self, x: f32, y: f32, text: &str, size: f32) {
        self.content.push(format!("BT"));
        self.content.push(format!("/F1 {} Tf", size));
        self.content.push(format!("{} {} Td", x, y));
        self.content.push(format!("({}) Tj", text.replace('(', "\\(").replace(')', "\\)")));
        self.content.push(format!("ET"));
    }

    pub fn add_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.content.push(format!("{} {} m", x1, y1));
        self.content.push(format!("{} {} l", x2, y2));
        self.content.push("S".to_string());
    }

    pub fn add_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.content.push(format!("{} {} {} {} re", x, y, width, height));
        self.content.push("S".to_string());
    }
}

// Visa application PDF generator
pub fn generate_visa_document(applicant_name: &str, output_path: &str) -> io::Result<()> {
    let mut doc = PdfDocument::new().with_metadata(PdfMetadata {
        title: "UAE Golden Visa Application".to_string(),
        author: applicant_name.to_string(),
        subject: "Entrepreneur Visa Application".to_string(),
        creator: "AvilaPdf/Dubai Project".to_string(),
    });

    let page = doc.add_page(595.0, 842.0); // A4 size
    
    page.add_text(50.0, 800.0, "UAE GOLDEN VISA APPLICATION", 24.0);
    page.add_text(50.0, 750.0, "Entrepreneur Category", 16.0);
    page.add_line(50.0, 740.0, 545.0, 740.0);
    
    page.add_text(50.0, 700.0, &format!("Applicant: {}", applicant_name), 12.0);
    page.add_text(50.0, 680.0, "Category: Entrepreneur (10-year visa)", 12.0);
    page.add_text(50.0, 660.0, "Investment Amount: AED 500,000+", 12.0);

    doc.save(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_pdf_creation() {
        let mut doc = PdfDocument::new();
        let page = doc.add_page(595.0, 842.0);
        page.add_text(100.0, 700.0, "Hello Dubai!", 12.0);
        
        let path = "test.pdf";
        assert!(doc.save(path).is_ok());
        assert!(fs::metadata(path).is_ok());
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_visa_document() {
        let path = "test_visa.pdf";
        assert!(generate_visa_document("Nícolas Ávila", path).is_ok());
        assert!(fs::metadata(path).is_ok());
        fs::remove_file(path).ok();
    }
}
