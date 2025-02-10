use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_956556503: FileType = FileType {
    file_format: &FileFormat {
        id: 956_556_503,
        source_type: SourceType::Linguist,
        name: "WebAssembly",
        extensions: &["wast", "wat"],
        media_types: &["text/x-common-lisp"],
        signatures: &[],
        related_formats: &[],
    },
};
