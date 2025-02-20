use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128779413: FileType = FileType {
    file_format: &FileFormat {
        id: 128_779_413,
        source_type: SourceType::Wikidata,
        name: "Cryptographic Protocol Shapes Analyzer file",
        extensions: &["cpsa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
