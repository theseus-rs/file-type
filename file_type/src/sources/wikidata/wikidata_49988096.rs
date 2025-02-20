use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49988096: FileType = FileType {
    file_format: &FileFormat {
        id: 49_988_096,
        source_type: SourceType::Wikidata,
        name: "Apple iBooks format",
        extensions: &["ibooks"],
        media_types: &["application/x-ibooks+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
