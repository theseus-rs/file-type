use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1935: FileType = FileType {
    file_format: &FileFormat {
        id: 1_935,
        source_type: SourceType::Pronom,
        name: "JASCO JWS Format",
        extensions: &["jws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
