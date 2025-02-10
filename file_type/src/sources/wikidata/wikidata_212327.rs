use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_212327: FileType = FileType {
    file_format: &FileFormat {
        id: 212_327,
        source_type: SourceType::Wikidata,
        name: "Document Type Definition",
        extensions: &["dtd"],
        media_types: &["application/xml-dtd"],
        signatures: &[],
        related_formats: &[],
    },
};
