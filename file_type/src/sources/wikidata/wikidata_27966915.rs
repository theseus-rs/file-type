use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27966915: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_915,
        source_type: SourceType::Wikidata,
        name: "NES Sound Format Extended",
        extensions: &["nsfe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
