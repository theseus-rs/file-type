use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_758803624: FileType = FileType {
    file_format: &FileFormat {
        id: 758_803_624,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
