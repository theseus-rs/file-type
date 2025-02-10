use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600238: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_238,
        source_type: SourceType::Wikidata,
        name: "ARC",
        extensions: &["arc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
