use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117835557: FileType = FileType {
    file_format: &FileFormat {
        id: 117_835_557,
        source_type: SourceType::Wikidata,
        name: "Knowledge Access file",
        extensions: &["cpr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
