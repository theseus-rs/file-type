use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3831603838: FileType = FileType {
    file_format: &FileFormat {
        id: 3_831_603_838,
        source_type: SourceType::Iana,
        name: "vnd.fvt",
        extensions: &[],
        media_types: &["video/vnd.fvt"],
        signatures: &[],
        related_formats: &[],
    },
};
