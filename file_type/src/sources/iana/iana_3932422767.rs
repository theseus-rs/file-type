use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3932422767: FileType = FileType {
    file_format: &FileFormat {
        id: 3_932_422_767,
        source_type: SourceType::Iana,
        name: "x-mixed-replace",
        extensions: &[],
        media_types: &["multipart/x-mixed-replace"],
        signatures: &[],
        related_formats: &[],
    },
};
