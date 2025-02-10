use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3963433202: FileType = FileType {
    file_format: &FileFormat {
        id: 3_963_433_202,
        source_type: SourceType::Iana,
        name: "vnd.apexlang",
        extensions: &[],
        media_types: &["application/vnd.apexlang"],
        signatures: &[],
        related_formats: &[],
    },
};
