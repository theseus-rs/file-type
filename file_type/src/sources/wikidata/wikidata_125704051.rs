use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125704051: FileType = FileType {
    file_format: &FileFormat {
        id: 125_704_051,
        source_type: SourceType::Wikidata,
        name: "StarDraw 2.0 file",
        extensions: &["sgv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
