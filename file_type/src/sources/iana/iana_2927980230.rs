use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2927980230: FileType = FileType {
    file_format: &FileFormat {
        id: 2_927_980_230,
        source_type: SourceType::Iana,
        name: "vnd.oma.group-usage-list+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.group-usage-list+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
