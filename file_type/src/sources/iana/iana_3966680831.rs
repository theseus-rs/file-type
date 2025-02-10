use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3966680831: FileType = FileType {
    file_format: &FileFormat {
        id: 3_966_680_831,
        source_type: SourceType::Iana,
        name: "ulpfec",
        extensions: &[],
        media_types: &["video/ulpfec"],
        signatures: &[],
        related_formats: &[],
    },
};
