use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117843653: FileType = FileType {
    file_format: &FileFormat {
        id: 117_843_653,
        source_type: SourceType::Wikidata,
        name: "IBM GOCA file",
        extensions: &["gca"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
