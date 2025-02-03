use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1675095755: FileFormat = FileFormat {
    id: 1_675_095_755,
    source_type: SourceType::Iana,
    name: "3gpdash-qoe-report+xml",
    extensions: &[],
    media_types: &["application/3gpdash-qoe-report+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
