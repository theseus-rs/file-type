use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121913987: FileType = FileType {
    file_format: &FileFormat {
        id: 121_913_987,
        source_type: SourceType::Wikidata,
        name: "Digital Voice File TRC Codec",
        extensions: &["dvf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
