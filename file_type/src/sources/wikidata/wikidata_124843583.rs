use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124843583: FileType = FileType {
    file_format: &FileFormat {
        id: 124_843_583,
        source_type: SourceType::Wikidata,
        name: "XTiger template",
        extensions: &["xtd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
