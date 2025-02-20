use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960082: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_082,
        source_type: SourceType::Wikidata,
        name: "DCT",
        extensions: &["dct", "wav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
