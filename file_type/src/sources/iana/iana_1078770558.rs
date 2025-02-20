use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1078770558: FileType = FileType {
    file_format: &FileFormat {
        id: 1_078_770_558,
        source_type: SourceType::Iana,
        name: "x-emf - DEPRECATED in favor of image/emf",
        extensions: &[],
        media_types: &["image/x-emf"],
        signatures: &[],
        related_formats: &[],
    },
};
