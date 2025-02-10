use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2866207433: FileType = FileType {
    file_format: &FileFormat {
        id: 2_866_207_433,
        source_type: SourceType::Httpd,
        name: "mediastation cdkey",
        extensions: &["cdkey"],
        media_types: &["application/vnd.mediastation.cdkey"],
        signatures: &[],
        related_formats: &[],
    },
};
