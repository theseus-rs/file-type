use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600435: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_435,
        source_type: SourceType::Wikidata,
        name: "Consolidated.db",
        extensions: &["db"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
