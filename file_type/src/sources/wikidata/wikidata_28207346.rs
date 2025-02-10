use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207346: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_346,
        source_type: SourceType::Wikidata,
        name: "Image Capture Board",
        extensions: &["icb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
