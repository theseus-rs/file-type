use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1660650536: FileFormat = FileFormat {
    id: 1_660_650_536,
    source_type: SourceType::Iana,
    name: "rtx",
    extensions: &[],
    media_types: &["application/rtx"],
    internal_signatures: &[],
    related_formats: &[],
};
