use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_904873: FileType = FileType {
    file_format: &FileFormat {
        id: 904_873,
        source_type: SourceType::Wikidata,
        name: "Cryptographic Message Syntax",
        extensions: &["cmsc"],
        media_types: &["application/cms"],
        signatures: &[],
        related_formats: &[],
    },
};
