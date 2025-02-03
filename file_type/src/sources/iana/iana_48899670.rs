use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_48899670: FileFormat = FileFormat {
    id: 48_899_670,
    source_type: SourceType::Iana,
    name: "jls",
    extensions: &[],
    media_types: &["image/jls"],
    internal_signatures: &[],
    related_formats: &[],
};
