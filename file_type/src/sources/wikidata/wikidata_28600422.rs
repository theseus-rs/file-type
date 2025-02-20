use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600422: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_422,
        source_type: SourceType::Wikidata,
        name: "4bottle",
        extensions: &["4q"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
