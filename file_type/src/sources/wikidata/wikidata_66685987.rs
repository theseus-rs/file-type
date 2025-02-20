use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66685987: FileType = FileType {
    file_format: &FileFormat {
        id: 66_685_987,
        source_type: SourceType::Wikidata,
        name: "OR4",
        extensions: &["or4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
