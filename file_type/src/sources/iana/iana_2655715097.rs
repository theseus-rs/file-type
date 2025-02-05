use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2655715097: FileFormat = FileFormat {
    id: 2_655_715_097,
    source_type: SourceType::Iana,
    name: "vnd.msgpack",
    extensions: &[],
    media_types: &["application/vnd.msgpack"],
    signatures: &[],
    related_formats: &[],
};
