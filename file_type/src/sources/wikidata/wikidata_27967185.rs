use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967185: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_185,
        source_type: SourceType::Wikidata,
        name: "Fuchs Tracker",
        extensions: &["fchs", "ft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
