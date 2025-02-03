use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2126647565: FileFormat = FileFormat {
    id: 2_126_647_565,
    source_type: SourceType::Iana,
    name: "vnd.xiff",
    extensions: &[],
    media_types: &["image/vnd.xiff"],
    internal_signatures: &[],
    related_formats: &[],
};
