use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28777715: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_715,
        source_type: SourceType::Wikidata,
        name: "NSIS",
        extensions: &["exe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
