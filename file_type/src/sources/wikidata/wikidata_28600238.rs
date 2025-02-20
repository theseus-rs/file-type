use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
