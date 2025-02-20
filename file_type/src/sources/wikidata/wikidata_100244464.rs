use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
