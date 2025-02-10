use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
