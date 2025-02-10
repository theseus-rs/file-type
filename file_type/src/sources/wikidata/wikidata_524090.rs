use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_524090: FileType = FileType {
    file_format: &FileFormat {
        id: 524_090,
        source_type: SourceType::Wikidata,
        name: "MT9",
        extensions: &["mt9"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
