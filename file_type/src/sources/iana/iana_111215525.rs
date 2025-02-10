use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_111215525: FileType = FileType {
    file_format: &FileFormat {
        id: 111_215_525,
        source_type: SourceType::Iana,
        name: "vnd.sealed.xls",
        extensions: &[],
        media_types: &["application/vnd.sealed.xls"],
        signatures: &[],
        related_formats: &[],
    },
};
