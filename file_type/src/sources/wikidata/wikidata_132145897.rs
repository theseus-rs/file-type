use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_132145897: FileType = FileType {
    file_format: &FileFormat {
        id: 132_145_897,
        source_type: SourceType::Wikidata,
        name: "Braille Ready Format",
        extensions: &["brf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
