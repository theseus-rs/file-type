use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3111558661: FileFormat = FileFormat {
    id: 3_111_558_661,
    source_type: SourceType::Iana,
    name: "senml-etch+cbor",
    extensions: &[],
    media_types: &["application/senml-etch+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
