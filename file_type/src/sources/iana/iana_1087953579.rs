use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1087953579: FileType = FileType {
    file_format: &FileFormat {
        id: 1_087_953_579,
        source_type: SourceType::Iana,
        name: "vnd.3gpp-prose+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp-prose+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
