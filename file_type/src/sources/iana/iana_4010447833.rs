use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4010447833: FileType = FileType {
    file_format: &FileFormat {
        id: 4_010_447_833,
        source_type: SourceType::Iana,
        name: "vnd.quarantainenet",
        extensions: &[],
        media_types: &["application/vnd.quarantainenet"],
        signatures: &[],
        related_formats: &[],
    },
};
