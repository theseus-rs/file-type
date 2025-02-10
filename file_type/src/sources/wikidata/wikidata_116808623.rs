use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116808623: FileType = FileType {
    file_format: &FileFormat {
        id: 116_808_623,
        source_type: SourceType::Wikidata,
        name: "WillMaker File",
        extensions: &["ww8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
