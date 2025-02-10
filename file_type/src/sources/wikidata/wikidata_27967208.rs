use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967208: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_208,
        source_type: SourceType::Wikidata,
        name: "Pro Tracker v2.xx module",
        extensions: &["pt2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
