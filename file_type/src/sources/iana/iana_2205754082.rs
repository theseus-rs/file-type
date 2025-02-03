use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2205754082: FileFormat = FileFormat {
    id: 2_205_754_082,
    source_type: SourceType::Iana,
    name: "xacml+xml",
    extensions: &[],
    media_types: &["application/xacml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
