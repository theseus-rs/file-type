use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650343: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_343,
        source_type: SourceType::Wikidata,
        name: "PEM encoded RSA private key",
        extensions: &["key", "pem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
