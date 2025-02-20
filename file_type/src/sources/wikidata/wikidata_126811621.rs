use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126811621: FileType = FileType {
    file_format: &FileFormat {
        id: 126_811_621,
        source_type: SourceType::Wikidata,
        name: "Bennu bitmap file",
        extensions: &["map"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
