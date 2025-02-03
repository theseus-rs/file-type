use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4010447833: FileFormat = FileFormat {
    id: 4_010_447_833,
    source_type: SourceType::Iana,
    name: "vnd.quarantainenet",
    extensions: &[],
    media_types: &["application/vnd.quarantainenet"],
    internal_signatures: &[],
    related_formats: &[],
};
