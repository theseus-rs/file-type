use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125926011: FileType = FileType {
    file_format: &FileFormat {
        id: 125_926_011,
        source_type: SourceType::Wikidata,
        name: "Final Writer Document",
        extensions: &["fw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
