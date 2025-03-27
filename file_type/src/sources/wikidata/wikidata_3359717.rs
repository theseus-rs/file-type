use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3359717: FileType = FileType {
    file_format: &FileFormat {
        id: 3_359_717,
        source_type: SourceType::Wikidata,
        name: "Public Key Cryptography Standard 12 file",
        extensions: &["p12", "pfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
