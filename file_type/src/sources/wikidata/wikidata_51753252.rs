use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51753252: FileType = FileType {
    file_format: &FileFormat {
        id: 51_753_252,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Compiled Shape/Font File",
        extensions: &["shx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
