use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_268959593: FileType = FileType {
    file_format: &FileFormat {
        id: 268_959_593,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcdata-regroup+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcdata-regroup+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
