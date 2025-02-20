use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959801: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_801,
        source_type: SourceType::Wikidata,
        name: "Ableton Groove File",
        extensions: &["agr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
