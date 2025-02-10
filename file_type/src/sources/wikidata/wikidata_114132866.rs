use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114132866: FileType = FileType {
    file_format: &FileFormat {
        id: 114_132_866,
        source_type: SourceType::Wikidata,
        name: "Connectivity Table file format",
        extensions: &["ct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
