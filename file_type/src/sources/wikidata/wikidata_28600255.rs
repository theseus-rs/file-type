use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600255: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_255,
        source_type: SourceType::Wikidata,
        name: "ARTS",
        extensions: &["arts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
