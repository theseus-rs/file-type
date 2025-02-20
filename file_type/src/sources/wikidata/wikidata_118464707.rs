use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118464707: FileType = FileType {
    file_format: &FileFormat {
        id: 118_464_707,
        source_type: SourceType::Wikidata,
        name: "Open Media Framework Interchange 1.0",
        extensions: &["omf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
