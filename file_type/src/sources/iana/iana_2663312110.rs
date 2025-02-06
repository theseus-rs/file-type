use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2663312110: FileFormat = FileFormat {
    id: 2_663_312_110,
    source_type: SourceType::Iana,
    name: "xcap-caps+xml",
    extensions: &[],
    media_types: &["application/xcap-caps+xml"],
    signatures: &[],
    related_formats: &[],
};
