use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1531716268: FileType = FileType {
    file_format: &FileFormat {
        id: 1_531_716_268,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.notification+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.notification+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
