use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2643175794: FileType = FileType {
    file_format: &FileFormat {
        id: 2_643_175_794,
        source_type: SourceType::Iana,
        name: "AV1",
        extensions: &[],
        media_types: &["video/AV1"],
        signatures: &[],
        related_formats: &[],
    },
};
