use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135728625: FileType = FileType {
    file_format: &FileFormat {
        id: 135_728_625,
        source_type: SourceType::Wikidata,
        name: "LEMON Graph Format",
        extensions: &["lgf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
