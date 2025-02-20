use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1018447787: FileType = FileType {
    file_format: &FileFormat {
        id: 1_018_447_787,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.base",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.base"],
        signatures: &[],
        related_formats: &[],
    },
};
