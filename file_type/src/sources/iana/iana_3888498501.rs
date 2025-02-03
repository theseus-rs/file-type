use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3888498501: FileFormat = FileFormat {
    id: 3_888_498_501,
    source_type: SourceType::Iana,
    name: "vnd.etsi.iptvservice+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.iptvservice+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
