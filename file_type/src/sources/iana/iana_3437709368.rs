use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3437709368: FileFormat = FileFormat {
    id: 3_437_709_368,
    source_type: SourceType::Iana,
    name: "vnd.syncml+xml",
    extensions: &[],
    media_types: &["application/vnd.syncml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
