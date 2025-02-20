use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206788: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_788,
        source_type: SourceType::Wikidata,
        name: "OS/2 Bitmap, version 2.0",
        extensions: &["bmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
