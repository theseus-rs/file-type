use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205667: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_667,
        source_type: SourceType::Wikidata,
        name: "Public Key Cryptography Standard 10",
        extensions: &["csr", "p10", "pem"],
        media_types: &["application/pkcs10"],
        signatures: &[],
        related_formats: &[],
    },
};
