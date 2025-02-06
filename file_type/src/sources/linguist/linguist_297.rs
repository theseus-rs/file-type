use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_297: FileFormat = FileFormat {
    id: 297,
    source_type: SourceType::Linguist,
    name: "Protocol Buffer",
    extensions: &["proto"],
    media_types: &["text/x-protobuf"],
    signatures: &[],
    related_formats: &[],
};
