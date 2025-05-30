use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28777715: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_715,
        source_type: SourceType::Wikidata,
        name: "NSIS file format",
        extensions: &["exe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
