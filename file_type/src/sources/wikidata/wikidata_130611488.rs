use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130611488: FileType = FileType {
    file_format: &FileFormat {
        id: 130_611_488,
        source_type: SourceType::Wikidata,
        name: "Red language file format",
        extensions: &["red", "reds"],
        media_types: &["text/x-red", "text/x-red-system"],
        signatures: &[],
        related_formats: &[],
    },
};
