use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136678464: FileType = FileType {
    file_format: &FileFormat {
        id: 136_678_464,
        source_type: SourceType::Wikidata,
        name: "Celtx, Screenwriting & Media Pre-production file",
        extensions: &["celtex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
