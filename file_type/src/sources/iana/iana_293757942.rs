use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_293757942: FileFormat = FileFormat {
    id: 293_757_942,
    source_type: SourceType::Iana,
    name: "vnd.xmi+xml",
    extensions: &[],
    media_types: &["application/vnd.xmi+xml"],
    signatures: &[],
    related_formats: &[],
};
