use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124843606: FileType = FileType {
    file_format: &FileFormat {
        id: 124_843_606,
        source_type: SourceType::Wikidata,
        name: "XTiger library",
        extensions: &["xtl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
