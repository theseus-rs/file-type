use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_441266579: FileType = FileType {
    file_format: &FileFormat {
        id: 441_266_579,
        source_type: SourceType::Iana,
        name: "vnd.objectvideo",
        extensions: &[],
        media_types: &["video/vnd.objectvideo"],
        signatures: &[],
        related_formats: &[],
    },
};
