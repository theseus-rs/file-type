use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105861080: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_080,
        source_type: SourceType::Wikidata,
        name: "Windows Application Log",
        extensions: &["lgc", "lgd"],
        media_types: &["application/json", "text/csv", "text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
