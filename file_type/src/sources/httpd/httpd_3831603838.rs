use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3831603838: FileType = FileType {
    file_format: &FileFormat {
        id: 3_831_603_838,
        source_type: SourceType::Httpd,
        name: "fvt",
        extensions: &["fvt"],
        media_types: &["video/vnd.fvt"],
        signatures: &[],
        related_formats: &[],
    },
};
