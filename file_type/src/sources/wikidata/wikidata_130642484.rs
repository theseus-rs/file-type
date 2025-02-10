use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130642484: FileType = FileType {
    file_format: &FileFormat {
        id: 130_642_484,
        source_type: SourceType::Wikidata,
        name: "Roboconf instances file",
        extensions: &["instances"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
