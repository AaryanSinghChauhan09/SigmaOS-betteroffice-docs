/// BetterOffice Docs: document model with paragraphs, inline styles, export.
#[derive(Debug, Clone, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub text: String,
    pub align: Align,
    pub bold: bool,
    pub italic: bool,
}

impl Paragraph {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            align: Align::Left,
            bold: false,
            italic: false,
        }
    }

    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    pub fn to_html(&self) -> String {
        let mut s = self.text.clone();
        if self.bold {
            s = format!("<b>{}</b>", s);
        }
        if self.italic {
            s = format!("<i>{}</i>", s);
        }
        s
    }
}

#[derive(Debug, Default)]
pub struct Document {
    pub title: String,
    pub paragraphs: Vec<Paragraph>,
}

impl Document {
    pub fn push(&mut self, p: Paragraph) {
        self.paragraphs.push(p);
    }
    pub fn word_count(&self) -> usize {
        self.paragraphs.iter().map(|p| p.word_count()).sum()
    }
    pub fn para_count(&self) -> usize {
        self.paragraphs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragraph_word_count_aaa() {
        let p = Paragraph::new("hello world foo");
        assert_eq!(p.word_count(), 3);
    }

    #[test]
    fn test_bold_paragraph_to_html_aaa() {
        let mut p = Paragraph::new("sigma");
        p.bold = true;
        assert_eq!(p.to_html(), "<b>sigma</b>");
    }

    #[test]
    fn test_document_total_word_count_aaa() {
        let mut doc = Document::default();
        doc.push(Paragraph::new("one two"));
        doc.push(Paragraph::new("three"));
        assert_eq!(doc.word_count(), 3);
    }

    #[test]
    fn test_document_para_count_aaa() {
        let mut doc = Document::default();
        doc.push(Paragraph::new("a"));
        doc.push(Paragraph::new("b"));
        assert_eq!(doc.para_count(), 2);
    }
}
