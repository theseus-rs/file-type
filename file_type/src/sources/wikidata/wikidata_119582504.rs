use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119582504: FileType = FileType {
    file_format: &FileFormat {
        id: 119_582_504,
        source_type: SourceType::Wikidata,
        name: "EMLX",
        extensions: &["emlx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
