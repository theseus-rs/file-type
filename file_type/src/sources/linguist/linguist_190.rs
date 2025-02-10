use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_190: FileType = FileType {
    file_format: &FileFormat {
        id: 190,
        source_type: SourceType::Linguist,
        name: "LFE",
        extensions: &["lfe"],
        media_types: &["text/x-common-lisp"],
        signatures: &[],
        related_formats: &[],
    },
};
