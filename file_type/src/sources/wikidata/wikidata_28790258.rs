use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28790258: FileType = FileType {
    file_format: &FileFormat {
        id: 28_790_258,
        source_type: SourceType::Wikidata,
        name: "maz",
        extensions: &["maz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
