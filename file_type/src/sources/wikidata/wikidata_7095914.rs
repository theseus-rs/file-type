use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7095914: FileType = FileType {
    file_format: &FileFormat {
        id: 7_095_914,
        source_type: SourceType::Wikidata,
        name: "OpenXDF",
        extensions: &["xdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
