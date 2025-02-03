use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4150371756: FileFormat = FileFormat {
    id: 4_150_371_756,
    source_type: SourceType::Iana,
    name: "vnd.futoin+cbor",
    extensions: &[],
    media_types: &["application/vnd.futoin+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
