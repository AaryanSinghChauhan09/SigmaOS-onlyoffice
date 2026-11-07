//! onlyoffice model — office suite (Document Server).

#[derive(Debug, Clone)]
pub struct Document {
    pub name: String,
    pub type_: String,
    pub path: String,
    pub shared: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub documents: Vec<Document>,
}

impl Default for Model {
    fn default() -> Self { Self { documents: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, doc: Document) { self.documents.push(doc); }

    pub fn share(&mut self, name: &str) -> bool {
        if let Some(d) = self.documents.iter_mut().find(|d| d.name == name) {
            d.shared = true; true
        } else { false }
    }

    pub fn list(&self) -> &[Document] { &self.documents }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_share() {
        let mut o = Model::new();
        o.open(Document { name: "doc1".into(), type_: "document".into(), path: "/doc1.docx".into(), shared: false });
        assert!(o.share("doc1"));
    }
}
