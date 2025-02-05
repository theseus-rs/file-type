use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_956556503: FileFormat = FileFormat {
    id: 956_556_503,
    source_type: SourceType::Linguist,
    name: "WebAssembly",
    extensions: &["wast", "wat"],
    media_types: &["text/x-common-lisp"],
    signatures: &[],
    related_formats: &[],
};
