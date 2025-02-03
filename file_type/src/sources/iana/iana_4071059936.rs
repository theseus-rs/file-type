use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4071059936: FileFormat = FileFormat {
    id: 4_071_059_936,
    source_type: SourceType::Iana,
    name: "global",
    extensions: &[],
    media_types: &["message/global"],
    internal_signatures: &[],
    related_formats: &[],
};
