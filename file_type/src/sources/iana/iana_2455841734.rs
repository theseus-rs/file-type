use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2455841734: FileFormat = FileFormat {
    id: 2_455_841_734,
    source_type: SourceType::Iana,
    name: "dash-patch+xml",
    extensions: &[],
    media_types: &["application/dash-patch+xml"],
    signatures: &[],
    related_formats: &[],
};
