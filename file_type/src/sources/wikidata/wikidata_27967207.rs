use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967207: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_207,
        source_type: SourceType::Wikidata,
        name: "Pro Tracker v1.xx module",
        extensions: &["pt1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
