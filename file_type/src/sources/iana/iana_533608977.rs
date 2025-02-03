use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_533608977: FileFormat = FileFormat {
    id: 533_608_977,
    source_type: SourceType::Iana,
    name: "patch-ops-error+xml",
    extensions: &[],
    media_types: &["application/patch-ops-error+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
