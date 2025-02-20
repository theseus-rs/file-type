use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118140187: FileType = FileType {
    file_format: &FileFormat {
        id: 118_140_187,
        source_type: SourceType::Wikidata,
        name: "Serenade Symbol File",
        extensions: &["sym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
