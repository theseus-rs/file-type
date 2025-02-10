use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4149901353: FileType = FileType {
    file_format: &FileFormat {
        id: 4_149_901_353,
        source_type: SourceType::Iana,
        name: "vnd.omads-email+xml",
        extensions: &[],
        media_types: &["application/vnd.omads-email+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
