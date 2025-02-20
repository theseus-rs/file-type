use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34745947: FileType = FileType {
    file_format: &FileFormat {
        id: 34_745_947,
        source_type: SourceType::Wikidata,
        name: "Starlink Data Format",
        extensions: &["sdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
