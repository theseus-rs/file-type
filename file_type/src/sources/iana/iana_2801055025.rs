use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2801055025: FileFormat = FileFormat {
    id: 2_801_055_025,
    source_type: SourceType::Iana,
    name: "H261",
    extensions: &[],
    media_types: &["video/H261"],
    internal_signatures: &[],
    related_formats: &[],
};
