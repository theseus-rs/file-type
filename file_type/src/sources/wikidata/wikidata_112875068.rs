use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112875068: FileType = FileType {
    file_format: &FileFormat {
        id: 112_875_068,
        source_type: SourceType::Wikidata,
        name: "armored PGP public key block",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
