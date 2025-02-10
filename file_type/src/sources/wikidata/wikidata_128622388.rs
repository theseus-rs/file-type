use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128622388: FileType = FileType {
    file_format: &FileFormat {
        id: 128_622_388,
        source_type: SourceType::Wikidata,
        name: "Augeas file format",
        extensions: &["aug"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
