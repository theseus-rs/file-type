use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_842202839: FileType = FileType {
    file_format: &FileFormat {
        id: 842_202_839,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
