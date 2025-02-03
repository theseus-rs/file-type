use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_190: FileFormat = FileFormat {
    id: 190,
    source_type: SourceType::Linguist,
    name: "LFE",
    extensions: &["lfe"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
