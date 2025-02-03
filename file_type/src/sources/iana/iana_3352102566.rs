use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3352102566: FileFormat = FileFormat {
    id: 3_352_102_566,
    source_type: SourceType::Iana,
    name: "H266",
    extensions: &[],
    media_types: &["video/H266"],
    internal_signatures: &[],
    related_formats: &[],
};
