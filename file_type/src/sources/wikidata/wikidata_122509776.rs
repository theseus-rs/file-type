use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122509776: FileType = FileType {
    file_format: &FileFormat {
        id: 122_509_776,
        source_type: SourceType::Wikidata,
        name: "Pretty Good Privacy public key ring data file",
        extensions: &["pubkr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
