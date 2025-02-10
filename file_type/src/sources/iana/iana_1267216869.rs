use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1267216869: FileType = FileType {
    file_format: &FileFormat {
        id: 1_267_216_869,
        source_type: SourceType::Iana,
        name: "vnd.CCTV",
        extensions: &[],
        media_types: &["video/vnd.CCTV"],
        signatures: &[],
        related_formats: &[],
    },
};
