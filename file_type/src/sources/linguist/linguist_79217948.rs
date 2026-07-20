use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_79217948: FileType = FileType {
    file_format: &FileFormat {
        id: 79_217_948,
        source_type: SourceType::Linguist,
        name: "ALGOL",
        extensions: &["alg"],
        media_types: &["text/x-pascal"],
        signatures: &[],
        related_formats: &[],
    },
};
