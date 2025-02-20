use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59608885: FileType = FileType {
    file_format: &FileFormat {
        id: 59_608_885,
        source_type: SourceType::Wikidata,
        name: "PKCS #7 Cryptographic message file",
        extensions: &["p7m"],
        media_types: &["application/pkcs7-mime"],
        signatures: &[],
        related_formats: &[],
    },
};
