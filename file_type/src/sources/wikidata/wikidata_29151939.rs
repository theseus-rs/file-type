use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29151939: FileType = FileType {
    file_format: &FileFormat {
        id: 29_151_939,
        source_type: SourceType::Wikidata,
        name: "Random Dot Software Graphic QDV",
        extensions: &["qdv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
