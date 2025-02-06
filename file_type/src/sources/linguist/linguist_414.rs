use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_414: FileFormat = FileFormat {
    id: 414,
    source_type: SourceType::Linguist,
    name: "edn",
    extensions: &["edn"],
    media_types: &["text/x-clojure"],
    signatures: &[],
    related_formats: &[],
};
