use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4100053721: FileFormat = FileFormat {
    id: 4_100_053_721,
    source_type: SourceType::Iana,
    name: "vnd.adobe.fxp",
    extensions: &[],
    media_types: &["application/vnd.adobe.fxp"],
    signatures: &[],
    related_formats: &[],
};
