use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967486: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_486,
        source_type: SourceType::Wikidata,
        name: "F4V",
        extensions: &["f4a", "f4b", "f4p", "f4v"],
        media_types: &["video/mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
