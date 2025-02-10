use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3966587680: FileType = FileType {
    file_format: &FileFormat {
        id: 3_966_587_680,
        source_type: SourceType::Iana,
        name: "vnd.directv.mpeg",
        extensions: &[],
        media_types: &["video/vnd.directv.mpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
