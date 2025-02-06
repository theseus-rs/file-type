use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3959787308: FileFormat = FileFormat {
    id: 3_959_787_308,
    source_type: SourceType::Iana,
    name: "vnd.dece.unspecified",
    extensions: &[],
    media_types: &["application/vnd.dece.unspecified"],
    signatures: &[],
    related_formats: &[],
};
