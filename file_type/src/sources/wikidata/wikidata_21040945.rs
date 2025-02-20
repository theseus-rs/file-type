use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21040945: FileType = FileType {
    file_format: &FileFormat {
        id: 21_040_945,
        source_type: SourceType::Wikidata,
        name: "Digitrakker format",
        extensions: &["mdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
