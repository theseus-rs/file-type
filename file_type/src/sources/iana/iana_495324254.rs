use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_495324254: FileType = FileType {
    file_format: &FileFormat {
        id: 495_324_254,
        source_type: SourceType::Iana,
        name: "mpeg4-iod",
        extensions: &[],
        media_types: &["application/mpeg4-iod"],
        signatures: &[],
        related_formats: &[],
    },
};
