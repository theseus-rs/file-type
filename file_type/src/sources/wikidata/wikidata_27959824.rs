use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959824: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_824,
        source_type: SourceType::Wikidata,
        name: "Ableton Skin File",
        extensions: &["ask"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
