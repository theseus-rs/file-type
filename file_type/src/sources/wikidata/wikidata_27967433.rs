use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967433: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_433,
        source_type: SourceType::Wikidata,
        name: "Bink Video, version 1",
        extensions: &["bik"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
