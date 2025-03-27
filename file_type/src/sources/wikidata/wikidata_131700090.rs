use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131700090: FileType = FileType {
    file_format: &FileFormat {
        id: 131_700_090,
        source_type: SourceType::Wikidata,
        name: "Q131700090",
        extensions: &[],
        media_types: &["model/x3d+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
