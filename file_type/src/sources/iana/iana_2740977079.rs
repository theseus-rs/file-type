use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2740977079: FileFormat = FileFormat {
    id: 2_740_977_079,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcdata-affiliation-command+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcdata-affiliation-command+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
