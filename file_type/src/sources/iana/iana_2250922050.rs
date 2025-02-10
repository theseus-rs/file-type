use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2250922050: FileType = FileType {
    file_format: &FileFormat {
        id: 2_250_922_050,
        source_type: SourceType::Iana,
        name: "vnd.fpx",
        extensions: &[],
        media_types: &["image/vnd.fpx"],
        signatures: &[],
        related_formats: &[],
    },
};
