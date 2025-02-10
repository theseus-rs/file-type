use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1970132485: FileType = FileType {
    file_format: &FileFormat {
        id: 1_970_132_485,
        source_type: SourceType::Iana,
        name: "rtx",
        extensions: &[],
        media_types: &["video/rtx"],
        signatures: &[],
        related_formats: &[],
    },
};
