use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_621277: FileType = FileType {
    file_format: &FileFormat {
        id: 621_277,
        source_type: SourceType::Wikidata,
        name: "Apple Lossless",
        extensions: &["caf", "m4a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
