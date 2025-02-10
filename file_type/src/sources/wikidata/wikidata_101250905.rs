use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_101250905: FileType = FileType {
    file_format: &FileFormat {
        id: 101_250_905,
        source_type: SourceType::Wikidata,
        name: ".piskel",
        extensions: &["piskel"],
        media_types: &["image/png+json"],
        signatures: &[],
        related_formats: &[],
    },
};
