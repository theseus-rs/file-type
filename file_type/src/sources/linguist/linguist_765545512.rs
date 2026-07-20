use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_765545512: FileType = FileType {
    file_format: &FileFormat {
        id: 765_545_512,
        source_type: SourceType::Linguist,
        name: "Blueprint",
        extensions: &["blp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
