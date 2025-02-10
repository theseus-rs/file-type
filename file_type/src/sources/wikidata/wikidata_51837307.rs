use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51837307: FileType = FileType {
    file_format: &FileFormat {
        id: 51_837_307,
        source_type: SourceType::Wikidata,
        name: "IBM DisplayWrite DCA Text File",
        extensions: &["dca"],
        media_types: &["application/dca-rft"],
        signatures: &[],
        related_formats: &[],
    },
};
