use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206771: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_771,
        source_type: SourceType::Wikidata,
        name: "OS/2 Bitmap Array",
        extensions: &["bga", "bmp", "ico"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
