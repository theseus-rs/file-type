use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207064: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_064,
        source_type: SourceType::Wikidata,
        name: "Portable Bitmap Format",
        extensions: &["pbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
