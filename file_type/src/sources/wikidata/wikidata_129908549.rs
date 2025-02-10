use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129908549: FileType = FileType {
    file_format: &FileFormat {
        id: 129_908_549,
        source_type: SourceType::Wikidata,
        name: "JAGS file format",
        extensions: &["jag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
