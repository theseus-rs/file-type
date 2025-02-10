use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27480264: FileType = FileType {
    file_format: &FileFormat {
        id: 27_480_264,
        source_type: SourceType::Wikidata,
        name: "7z, version 0.2 (with compression methods version 4.65)",
        extensions: &["7z"],
        media_types: &["application/x-7z-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
