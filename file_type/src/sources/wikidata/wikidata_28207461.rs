use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207461: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_461,
        source_type: SourceType::Wikidata,
        name: "VITec Image Format",
        extensions: &["vit", "vitec"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
