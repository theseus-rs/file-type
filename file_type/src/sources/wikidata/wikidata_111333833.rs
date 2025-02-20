use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333833: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_833,
        source_type: SourceType::Wikidata,
        name: "Signed 8-bit PCM data",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
