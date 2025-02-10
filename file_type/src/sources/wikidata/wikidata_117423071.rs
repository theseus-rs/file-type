use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117423071: FileType = FileType {
    file_format: &FileFormat {
        id: 117_423_071,
        source_type: SourceType::Wikidata,
        name: "Stimulus file",
        extensions: &["stm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
