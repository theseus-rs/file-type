use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123360066: FileType = FileType {
    file_format: &FileFormat {
        id: 123_360_066,
        source_type: SourceType::Wikidata,
        name: "PHP 4 Web Page",
        extensions: &["php4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
