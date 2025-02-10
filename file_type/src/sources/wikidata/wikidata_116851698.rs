use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116851698: FileType = FileType {
    file_format: &FileFormat {
        id: 116_851_698,
        source_type: SourceType::Wikidata,
        name: "VersaCheck Data File",
        extensions: &["vdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
