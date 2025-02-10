use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_502801485: FileType = FileType {
    file_format: &FileFormat {
        id: 502_801_485,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
