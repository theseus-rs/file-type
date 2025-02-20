use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117835020: FileType = FileType {
    file_format: &FileFormat {
        id: 117_835_020,
        source_type: SourceType::Wikidata,
        name: "Canon Navigator Fax file",
        extensions: &["can"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
