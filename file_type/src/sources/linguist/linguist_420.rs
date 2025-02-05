use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_420: FileFormat = FileFormat {
    id: 420,
    source_type: SourceType::Linguist,
    name: "wisp",
    extensions: &["wisp"],
    media_types: &["text/x-clojure"],
    signatures: &[],
    related_formats: &[],
};
