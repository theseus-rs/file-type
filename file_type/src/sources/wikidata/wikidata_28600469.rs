use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600469: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_469,
        source_type: SourceType::Wikidata,
        name: "Distinguished Encoding Rules",
        extensions: &["der"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
