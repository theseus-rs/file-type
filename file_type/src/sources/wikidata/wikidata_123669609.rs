use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123669609: FileType = FileType {
    file_format: &FileFormat {
        id: 123_669_609,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing X8",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
