use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4288107160: FileFormat = FileFormat {
    id: 4_288_107_160,
    source_type: SourceType::Iana,
    name: "cbor",
    extensions: &[],
    media_types: &["application/cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
