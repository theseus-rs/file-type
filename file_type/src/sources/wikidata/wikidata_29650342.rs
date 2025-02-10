use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650342: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_342,
        source_type: SourceType::Wikidata,
        name: "PEM encoded certificate",
        extensions: &["cer", "crt", "pem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
