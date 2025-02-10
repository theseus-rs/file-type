use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1211156865: FileType = FileType {
    file_format: &FileFormat {
        id: 1_211_156_865,
        source_type: SourceType::Iana,
        name: "vnd.miele+json",
        extensions: &[],
        media_types: &["application/vnd.miele+json"],
        signatures: &[],
        related_formats: &[],
    },
};
