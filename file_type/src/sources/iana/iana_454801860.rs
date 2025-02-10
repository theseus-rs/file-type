use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_454801860: FileType = FileType {
    file_format: &FileFormat {
        id: 454_801_860,
        source_type: SourceType::Iana,
        name: "vnd.fst",
        extensions: &[],
        media_types: &["image/vnd.fst"],
        signatures: &[],
        related_formats: &[],
    },
};
