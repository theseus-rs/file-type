use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2831585141: FileFormat = FileFormat {
    id: 2_831_585_141,
    source_type: SourceType::Iana,
    name: "mads+xml",
    extensions: &[],
    media_types: &["application/mads+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
