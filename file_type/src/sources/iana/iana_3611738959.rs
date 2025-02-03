use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3611738959: FileFormat = FileFormat {
    id: 3_611_738_959,
    source_type: SourceType::Iana,
    name: "eat-bun+cbor",
    extensions: &[],
    media_types: &["application/eat-bun+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
