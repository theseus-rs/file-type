use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_543755811: FileFormat = FileFormat {
    id: 543_755_811,
    source_type: SourceType::Iana,
    name: "vnd.etsi.timestamp-token",
    extensions: &[],
    media_types: &["application/vnd.etsi.timestamp-token"],
    internal_signatures: &[],
    related_formats: &[],
};
