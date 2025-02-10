use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121158020: FileType = FileType {
    file_format: &FileFormat {
        id: 121_158_020,
        source_type: SourceType::Wikidata,
        name: "Letter file",
        extensions: &["rtf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
