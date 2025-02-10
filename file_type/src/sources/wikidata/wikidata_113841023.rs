use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113841023: FileType = FileType {
    file_format: &FileFormat {
        id: 113_841_023,
        source_type: SourceType::Wikidata,
        name: "JIFF",
        extensions: &["jif", "jiff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
