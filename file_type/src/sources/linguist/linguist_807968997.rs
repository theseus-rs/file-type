use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_807968997: FileType = FileType {
    file_format: &FileFormat {
        id: 807_968_997,
        source_type: SourceType::Linguist,
        name: "Git Config",
        extensions: &["gitconfig"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
