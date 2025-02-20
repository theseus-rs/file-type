use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967405: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_405,
        source_type: SourceType::Wikidata,
        name: "Master Tracker module",
        extensions: &["mtr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
