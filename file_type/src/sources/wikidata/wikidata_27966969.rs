use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966969: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_969,
        source_type: SourceType::Wikidata,
        name: "Crystal Caves Sound format",
        extensions: &["snd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
