use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_21041556: FileType = FileType {
    file_format: &FileFormat {
        id: 21_041_556,
        source_type: SourceType::Wikidata,
        name: "Music Editor format",
        extensions: &["med"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
