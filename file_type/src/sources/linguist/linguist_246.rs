use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_246: FileFormat = FileFormat {
    id: 246,
    source_type: SourceType::Linguist,
    name: "NetLogo",
    extensions: &["nlogo"],
    media_types: &["text/x-common-lisp"],
    signatures: &[],
    related_formats: &[],
};
