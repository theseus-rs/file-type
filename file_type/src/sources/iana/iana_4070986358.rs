use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4070986358: FileType = FileType {
    file_format: &FileFormat {
        id: 4_070_986_358,
        source_type: SourceType::Iana,
        name: "vnd.chess-pgn",
        extensions: &[],
        media_types: &["application/vnd.chess-pgn"],
        signatures: &[],
        related_formats: &[],
    },
};
