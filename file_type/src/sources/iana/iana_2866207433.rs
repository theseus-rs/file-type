use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2866207433: FileType = FileType {
    file_format: &FileFormat {
        id: 2_866_207_433,
        source_type: SourceType::Iana,
        name: "vnd.mediastation.cdkey",
        extensions: &[],
        media_types: &["application/vnd.mediastation.cdkey"],
        signatures: &[],
        related_formats: &[],
    },
};
