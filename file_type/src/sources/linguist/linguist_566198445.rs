use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_566198445: FileType = FileType {
    file_format: &FileFormat {
        id: 566_198_445,
        source_type: SourceType::Linguist,
        name: "mdsvex",
        extensions: &["svx"],
        media_types: &["text/x-gfm"],
        signatures: &[],
        related_formats: &[],
    },
};
