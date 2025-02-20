use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967102: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_102,
        source_type: SourceType::Wikidata,
        name: "Nintendo GameCube/Wii AST",
        extensions: &["ast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
