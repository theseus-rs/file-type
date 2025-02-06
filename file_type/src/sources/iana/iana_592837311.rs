use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_592837311: FileFormat = FileFormat {
    id: 592_837_311,
    source_type: SourceType::Iana,
    name: "simple-filter+xml",
    extensions: &[],
    media_types: &["application/simple-filter+xml"],
    signatures: &[],
    related_formats: &[],
};
