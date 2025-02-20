use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128034054: FileType = FileType {
    file_format: &FileFormat {
        id: 128_034_054,
        source_type: SourceType::Wikidata,
        name: "Rebol script",
        extensions: &["r"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
