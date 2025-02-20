use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
