use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_121: FileType = FileType {
    file_format: &FileFormat {
        id: 121,
        source_type: SourceType::Linguist,
        name: "GCC Machine Description",
        extensions: &["md"],
        media_types: &["text/x-common-lisp"],
        signatures: &[],
        related_formats: &[],
    },
};
