use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2542359355: FileFormat = FileFormat {
    id: 2_542_359_355,
    source_type: SourceType::Iana,
    name: "dots+cbor",
    extensions: &[],
    media_types: &["application/dots+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
