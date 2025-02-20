use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_935809: FileType = FileType {
    file_format: &FileFormat {
        id: 935_809,
        source_type: SourceType::Wikidata,
        name: "comma-separated values",
        extensions: &["csv"],
        media_types: &["text/csv"],
        signatures: &[],
        related_formats: &[],
    },
};
