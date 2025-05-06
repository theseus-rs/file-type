use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7532980: FileType = FileType {
    file_format: &FileFormat {
        id: 7_532_980,
        source_type: SourceType::Wikidata,
        name: "Sixel",
        extensions: &["six", "sixel"],
        media_types: &["image/x-sixel"],
        signatures: &[],
        related_formats: &[],
    },
};
