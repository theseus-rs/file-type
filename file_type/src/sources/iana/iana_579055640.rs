use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_579055640: FileFormat = FileFormat {
    id: 579_055_640,
    source_type: SourceType::Iana,
    name: "dsr-es201108",
    extensions: &[],
    media_types: &["audio/dsr-es201108"],
    internal_signatures: &[],
    related_formats: &[],
};
