use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111316769: FileType = FileType {
    file_format: &FileFormat {
        id: 111_316_769,
        source_type: SourceType::Wikidata,
        name: "Impulse Tracker instrument",
        extensions: &["iti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
