use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1071533059: FileType = FileType {
    file_format: &FileFormat {
        id: 1_071_533_059,
        source_type: SourceType::Iana,
        name: "vnd.stepmania.package",
        extensions: &[],
        media_types: &["application/vnd.stepmania.package"],
        signatures: &[],
        related_formats: &[],
    },
};
