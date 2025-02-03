use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_121: FileFormat = FileFormat {
    id: 121,
    source_type: SourceType::Linguist,
    name: "GCC Machine Description",
    extensions: &["md"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
