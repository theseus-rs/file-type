use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1806405536: FileFormat = FileFormat {
    id: 1_806_405_536,
    source_type: SourceType::Iana,
    name: "vnd.etsi.iptvdiscovery+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.iptvdiscovery+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
