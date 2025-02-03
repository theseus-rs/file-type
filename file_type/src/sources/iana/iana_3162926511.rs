use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3162926511: FileFormat = FileFormat {
    id: 3_162_926_511,
    source_type: SourceType::Iana,
    name: "pkcs12",
    extensions: &[],
    media_types: &["application/pkcs12"],
    internal_signatures: &[],
    related_formats: &[],
};
