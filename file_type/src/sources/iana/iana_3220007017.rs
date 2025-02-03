use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3220007017: FileFormat = FileFormat {
    id: 3_220_007_017,
    source_type: SourceType::Iana,
    name: "pointer",
    extensions: &[],
    media_types: &["video/pointer"],
    internal_signatures: &[],
    related_formats: &[],
};
