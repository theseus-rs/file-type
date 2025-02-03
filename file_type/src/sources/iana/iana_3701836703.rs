use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3701836703: FileFormat = FileFormat {
    id: 3_701_836_703,
    source_type: SourceType::Iana,
    name: "vnd.openstreetmap.data+xml",
    extensions: &[],
    media_types: &["application/vnd.openstreetmap.data+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
