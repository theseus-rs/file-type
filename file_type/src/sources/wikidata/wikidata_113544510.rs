use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113544510: FileType = FileType {
    file_format: &FileFormat {
        id: 113_544_510,
        source_type: SourceType::Wikidata,
        name: "PowerGraphics Image File",
        extensions: &["pgr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
