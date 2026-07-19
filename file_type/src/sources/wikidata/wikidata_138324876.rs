use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138324876: FileType = FileType {
    file_format: &FileFormat {
        id: 138_324_876,
        source_type: SourceType::Wikidata,
        name: "POWDER-S",
        extensions: &["rdf"],
        media_types: &["application/powder-s+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
