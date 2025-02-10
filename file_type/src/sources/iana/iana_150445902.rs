use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_150445902: FileType = FileType {
    file_format: &FileFormat {
        id: 150_445_902,
        source_type: SourceType::Iana,
        name: "vnd.onepagertatx",
        extensions: &[],
        media_types: &["application/vnd.onepagertatx"],
        signatures: &[],
        related_formats: &[],
    },
};
