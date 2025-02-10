use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207447: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_447,
        source_type: SourceType::Wikidata,
        name: "VIPS",
        extensions: &["v"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
