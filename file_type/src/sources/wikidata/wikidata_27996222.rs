use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27996222: FileType = FileType {
    file_format: &FileFormat {
        id: 27_996_222,
        source_type: SourceType::Wikidata,
        name: "Fallout character description",
        extensions: &["gcd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
