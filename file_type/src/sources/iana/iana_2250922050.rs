use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2250922050: FileFormat = FileFormat {
    id: 2_250_922_050,
    source_type: SourceType::Iana,
    name: "vnd.fpx",
    extensions: &[],
    media_types: &["image/vnd.fpx"],
    internal_signatures: &[],
    related_formats: &[],
};
