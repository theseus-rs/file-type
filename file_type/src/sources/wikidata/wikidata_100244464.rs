use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100244464: FileType = FileType {
    file_format: &FileFormat {
        id: 100_244_464,
        source_type: SourceType::Wikidata,
        name: "Student Writing Center Letter",
        extensions: &["lt", "ltt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
