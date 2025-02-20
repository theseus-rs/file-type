use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600454: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_454,
        source_type: SourceType::Wikidata,
        name: "DB (Watcom-SQL)",
        extensions: &["db"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
