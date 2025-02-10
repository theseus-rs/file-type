use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3741954711: FileType = FileType {
    file_format: &FileFormat {
        id: 3_741_954_711,
        source_type: SourceType::Iana,
        name: "vnd.las",
        extensions: &[],
        media_types: &["application/vnd.las"],
        signatures: &[],
        related_formats: &[],
    },
};
