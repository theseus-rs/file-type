use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_495324254: FileFormat = FileFormat {
    id: 495_324_254,
    source_type: SourceType::Iana,
    name: "mpeg4-iod",
    extensions: &[],
    media_types: &["application/mpeg4-iod"],
    signatures: &[],
    related_formats: &[],
};
