use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_505160760: FileFormat = FileFormat {
    id: 505_160_760,
    source_type: SourceType::Iana,
    name: "vnd.tmd.mediaflex.api+xml",
    extensions: &[],
    media_types: &["application/vnd.tmd.mediaflex.api+xml"],
    signatures: &[],
    related_formats: &[],
};
