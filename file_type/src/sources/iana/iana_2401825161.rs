use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2401825161: FileFormat = FileFormat {
    id: 2_401_825_161,
    source_type: SourceType::Iana,
    name: "avif",
    extensions: &[],
    media_types: &["image/avif"],
    internal_signatures: &[],
    related_formats: &[],
};
