use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123298073: FileType = FileType {
    file_format: &FileFormat {
        id: 123_298_073,
        source_type: SourceType::Wikidata,
        name: "Lotus Organizer 2.x/97 mapping",
        extensions: &["csv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
