use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967196: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_196,
        source_type: SourceType::Wikidata,
        name: "Impulse Tracker sample",
        extensions: &["its"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
