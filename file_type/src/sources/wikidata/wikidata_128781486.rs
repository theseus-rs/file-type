use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128781486: FileType = FileType {
    file_format: &FileFormat {
        id: 128_781_486,
        source_type: SourceType::Wikidata,
        name: "Cryptol file format",
        extensions: &["cry"],
        media_types: &["text/x-cryptol"],
        signatures: &[],
        related_formats: &[],
    },
};
