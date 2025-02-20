use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123360595: FileType = FileType {
    file_format: &FileFormat {
        id: 123_360_595,
        source_type: SourceType::Wikidata,
        name: "PHP 5 Web Page",
        extensions: &["php5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
