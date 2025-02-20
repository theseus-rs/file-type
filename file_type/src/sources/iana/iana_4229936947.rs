use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4229936947: FileType = FileType {
    file_format: &FileFormat {
        id: 4_229_936_947,
        source_type: SourceType::Iana,
        name: "vnd.shana.informed.interchange",
        extensions: &[],
        media_types: &["application/vnd.shana.informed.interchange"],
        signatures: &[],
        related_formats: &[],
    },
};
