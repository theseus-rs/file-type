use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1151597811: FileFormat = FileFormat {
    id: 1_151_597_811,
    source_type: SourceType::Iana,
    name: "ktx",
    extensions: &[],
    media_types: &["image/ktx"],
    internal_signatures: &[],
    related_formats: &[],
};
