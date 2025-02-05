use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_257856279: FileFormat = FileFormat {
    id: 257_856_279,
    source_type: SourceType::Linguist,
    name: "GSC",
    extensions: &["csc", "gsc", "gsh"],
    media_types: &["text/x-csrc"],
    signatures: &[],
    related_formats: &[],
};
