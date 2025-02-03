use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_960549232: FileFormat = FileFormat {
    id: 960_549_232,
    source_type: SourceType::Iana,
    name: "vnd.syncml.dm.notification",
    extensions: &[],
    media_types: &["application/vnd.syncml.dm.notification"],
    internal_signatures: &[],
    related_formats: &[],
};
