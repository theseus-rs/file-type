use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2496902356: FileFormat = FileFormat {
    id: 2_496_902_356,
    source_type: SourceType::Iana,
    name: "soap+xml",
    extensions: &[],
    media_types: &["application/soap+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
