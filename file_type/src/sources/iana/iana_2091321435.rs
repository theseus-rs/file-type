use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2091321435: FileFormat = FileFormat {
    id: 2_091_321_435,
    source_type: SourceType::Iana,
    name: "vnd.windows.devicepairing",
    extensions: &[],
    media_types: &["application/vnd.windows.devicepairing"],
    internal_signatures: &[],
    related_formats: &[],
};
