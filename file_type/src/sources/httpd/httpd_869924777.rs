use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_869924777: FileType = FileType {
    file_format: &FileFormat {
        id: 869_924_777,
        source_type: SourceType::Httpd,
        name: "xwindowdump",
        extensions: &["xwd"],
        media_types: &["image/x-xwindowdump"],
        signatures: &[],
        related_formats: &[],
    },
};
