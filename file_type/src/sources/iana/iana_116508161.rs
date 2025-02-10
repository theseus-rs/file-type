use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_116508161: FileType = FileType {
    file_format: &FileFormat {
        id: 116_508_161,
        source_type: SourceType::Iana,
        name: "mosskey-request",
        extensions: &[],
        media_types: &["application/mosskey-request"],
        signatures: &[],
        related_formats: &[],
    },
};
