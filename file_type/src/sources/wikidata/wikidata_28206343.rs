use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206343: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_343,
        source_type: SourceType::Wikidata,
        name: "ImgStar",
        extensions: &["cpx", "flt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
