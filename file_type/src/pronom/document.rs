use crate::pronom::serde::{deserialize_option_naive_date, serialize_option_naive_date};
use crate::pronom::{Author, DocumentIdentifier, Publisher};
use jiff::civil::Date;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Document {
    #[serde(rename = "DocumentID")]
    id: usize,
    display_text: String,
    document_type: String,
    availability_description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    availability_note: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_naive_date",
        serialize_with = "serialize_option_naive_date"
    )]
    publication_date: Option<Date>,
    title_text: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "DocumentIpr")]
    ipr: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "DocumentNote")]
    note: String,
    #[serde(rename = "DocumentIdentifier")]
    identifier: DocumentIdentifier,
    #[serde(rename = "Author")]
    authors: Vec<Author>,
    publisher: Publisher,
}

impl Document {
    /// Create a new document
    #[expect(clippy::too_many_arguments)]
    pub fn new<S: AsRef<str>>(
        id: usize,
        display_text: S,
        document_type: S,
        availability_description: S,
        availability_note: S,
        publication_date: Option<Date>,
        title_text: S,
        ipr: S,
        note: S,
        identifier: DocumentIdentifier,
        authors: Vec<Author>,
        publisher: Publisher,
    ) -> Self {
        Self {
            id,
            display_text: display_text.as_ref().to_string(),
            document_type: document_type.as_ref().to_string(),
            availability_description: availability_description.as_ref().to_string(),
            availability_note: availability_note.as_ref().to_string(),
            publication_date,
            title_text: title_text.as_ref().to_string(),
            ipr: ipr.as_ref().to_string(),
            note: note.as_ref().to_string(),
            identifier,
            authors,
            publisher,
        }
    }

    /// Get the ID of the document
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the display text of the document
    #[must_use]
    pub fn display_text(&self) -> &str {
        &self.display_text
    }

    /// Get the document type
    #[must_use]
    pub fn document_type(&self) -> &str {
        &self.document_type
    }

    /// Get the availability description of the document
    #[must_use]
    pub fn availability_description(&self) -> &str {
        &self.availability_description
    }

    /// Get the availability note of the document
    #[must_use]
    pub fn availability_note(&self) -> &str {
        &self.availability_note
    }

    /// Get the publication date of the document
    #[must_use]
    pub fn publication_date(&self) -> Option<Date> {
        self.publication_date
    }

    /// Get the title text of the document
    #[must_use]
    pub fn title_text(&self) -> &str {
        &self.title_text
    }

    /// Get the intellectual property rights of the document
    #[must_use]
    pub fn ipr(&self) -> &str {
        &self.ipr
    }

    /// Get the note of the document
    #[must_use]
    pub fn note(&self) -> &str {
        &self.note
    }

    /// Get the document identifier
    #[must_use]
    pub fn identifier(&self) -> &DocumentIdentifier {
        &self.identifier
    }

    /// Get the author of the document
    #[must_use]
    pub fn authors(&self) -> &[Author] {
        &self.authors
    }

    /// Get the publisher of the document
    #[must_use]
    pub fn publisher(&self) -> &Publisher {
        &self.publisher
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use indoc::indoc;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn test_serde() -> Result<()> {
        let xml = indoc! {r"
          <Document>
            <DocumentID>11</DocumentID>
            <DisplayText>OpenOffice.org, 2004, OpenOffice.org's documentation of the Microsoft Compound Document file format</DisplayText>
            <DocumentType>Speculative</DocumentType>
            <AvailabilityDescription>Public</AvailabilityDescription>
            <AvailabilityNote>
            </AvailabilityNote>
            <PublicationDate>15 Sep 2004</PublicationDate>
            <TitleText>OpenOffice.org's documentation of the Microsoft Compound Document file format</TitleText>
            <DocumentIPR>
            </DocumentIPR>
            <DocumentNote>
            </DocumentNote>
            <DocumentIdentifier>
              <Identifier>sc.openoffice.org/compdocfileformat.pdf</Identifier>
              <IdentifierType>URL</IdentifierType>
            </DocumentIdentifier>
            <Author>
              <AuthorID>103</AuthorID>
              <AuthorName>
              </AuthorName>
              <OrganisationName>OpenOffice.org</OrganisationName>
              <AuthorCompoundName>OpenOffice.org</AuthorCompoundName>
            </Author>
            <Publisher>
              <PublisherID>103</PublisherID>
              <PublisherName>
              </PublisherName>
              <OrganisationName>OpenOffice.org</OrganisationName>
              <PublisherCompoundName>OpenOffice.org</PublisherCompoundName>
            </Publisher>
          </Document>
        "};
        let document: Document = from_str(xml)?;

        // Test serialization
        let xml = to_string(&document)?;
        let document: Document = from_str(xml.as_str())?;

        assert_eq!(document.id(), 11);
        assert_eq!(document.display_text(), "OpenOffice.org, 2004, OpenOffice.org's documentation of the Microsoft Compound Document file format");
        assert_eq!(document.document_type(), "Speculative");
        assert_eq!(document.availability_description(), "Public");
        assert_eq!(document.availability_note(), "");
        assert_eq!(document.publication_date(), Date::new(2004, 9, 15).ok());
        assert_eq!(
            document.title_text(),
            "OpenOffice.org's documentation of the Microsoft Compound Document file format"
        );
        assert_eq!(document.ipr(), "");
        assert_eq!(document.note(), "");

        let identifier = document.identifier();
        assert_eq!(
            identifier.identifier(),
            "sc.openoffice.org/compdocfileformat.pdf"
        );
        assert_eq!(identifier.r#type(), "URL");

        let authors = document.authors();
        assert_eq!(authors.len(), 1);
        let author = &authors[0];
        assert_eq!(author.id(), 103);
        assert_eq!(author.name(), "");
        assert_eq!(author.organisation_name(), "OpenOffice.org");
        assert_eq!(author.compound_name(), "OpenOffice.org");

        let publisher = document.publisher();
        assert_eq!(publisher.id(), 103);
        assert_eq!(publisher.name(), "");
        assert_eq!(publisher.organisation_name(), "OpenOffice.org");
        assert_eq!(publisher.compound_name(), "OpenOffice.org");
        Ok(())
    }

    #[test]
    fn test_new() {
        let document = Document::new(
            11,
            "OpenOffice.org, 2004, OpenOffice.org's documentation of the Microsoft Compound Document file format",
            "Speculative",
            "Public",
            "",
            Date::new(2004, 9, 15).ok(),
            "OpenOffice.org's documentation of the Microsoft Compound Document file format",
            "",
            "",
            DocumentIdentifier::new("sc.openoffice.org/compdocfileformat.pdf", "URL"),
            vec![Author::new(103, "", "OpenOffice.org", "OpenOffice.org")],
            Publisher::new(103, "", "OpenOffice.org", "OpenOffice.org"),
        );
        assert_eq!(document.id(), 11);
        assert_eq!(document.display_text(), "OpenOffice.org, 2004, OpenOffice.org's documentation of the Microsoft Compound Document file format");
        assert_eq!(document.document_type(), "Speculative");
        assert_eq!(document.availability_description(), "Public");
        assert_eq!(document.availability_note(), "");
        assert_eq!(document.publication_date(), Date::new(2004, 9, 15).ok());
        assert_eq!(
            document.title_text(),
            "OpenOffice.org's documentation of the Microsoft Compound Document file format"
        );
        assert_eq!(document.ipr(), "");
        assert_eq!(document.note(), "");

        let identifier = document.identifier();
        assert_eq!(
            identifier.identifier(),
            "sc.openoffice.org/compdocfileformat.pdf"
        );
        assert_eq!(identifier.r#type(), "URL");

        let authors = document.authors();
        assert_eq!(authors.len(), 1);
        let author = &authors[0];
        assert_eq!(author.id(), 103);
        assert_eq!(author.name(), "");
        assert_eq!(author.organisation_name(), "OpenOffice.org");
        assert_eq!(author.compound_name(), "OpenOffice.org");

        let publisher = document.publisher();
        assert_eq!(publisher.id(), 103);
        assert_eq!(publisher.name(), "");
        assert_eq!(publisher.organisation_name(), "OpenOffice.org");
        assert_eq!(publisher.compound_name(), "OpenOffice.org");
    }
}
