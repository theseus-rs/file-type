use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1438392271: FileType = FileType {
    file_format: &FileFormat {
        id: 1_438_392_271,
        source_type: SourceType::Iana,
        name: "jpeg2000",
        extensions: &[],
        media_types: &["video/jpeg2000"],
        signatures: &[],
        related_formats: &[],
    },
};
