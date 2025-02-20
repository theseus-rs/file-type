use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132156191: FileType = FileType {
    file_format: &FileFormat {
        id: 132_156_191,
        source_type: SourceType::Wikidata,
        name: "NIMAS zipped file",
        extensions: &["zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
