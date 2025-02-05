use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1332246415: FileFormat = FileFormat {
    id: 1_332_246_415,
    source_type: SourceType::Iana,
    name: "x-wmf - DEPRECATED in favor of image/wmf",
    extensions: &[],
    media_types: &["image/x-wmf"],
    signatures: &[],
    related_formats: &[],
};
