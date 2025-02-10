use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123669580: FileType = FileType {
    file_format: &FileFormat {
        id: 123_669_580,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing X7",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
