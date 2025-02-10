use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28770329: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_329,
        source_type: SourceType::Wikidata,
        name: "Lightwave 3D Object",
        extensions: &["lwo"],
        media_types: &["application/x-lightwave", "image/x-lwo"],
        signatures: &[],
        related_formats: &[],
    },
};
