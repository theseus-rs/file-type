use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1066913277: FileType = FileType {
    file_format: &FileFormat {
        id: 1_066_913_277,
        source_type: SourceType::Iana,
        name: "vnd.ncd.control",
        extensions: &[],
        media_types: &["application/vnd.ncd.control"],
        signatures: &[],
        related_formats: &[],
    },
};
