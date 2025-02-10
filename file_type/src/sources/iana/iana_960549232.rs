use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_960549232: FileType = FileType {
    file_format: &FileFormat {
        id: 960_549_232,
        source_type: SourceType::Iana,
        name: "vnd.syncml.dm.notification",
        extensions: &[],
        media_types: &["application/vnd.syncml.dm.notification"],
        signatures: &[],
        related_formats: &[],
    },
};
