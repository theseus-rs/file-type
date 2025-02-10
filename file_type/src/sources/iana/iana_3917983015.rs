use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3917983015: FileType = FileType {
    file_format: &FileFormat {
        id: 3_917_983_015,
        source_type: SourceType::Iana,
        name: "vnd.dolby.mobile.2",
        extensions: &[],
        media_types: &["application/vnd.dolby.mobile.2"],
        signatures: &[],
        related_formats: &[],
    },
};
