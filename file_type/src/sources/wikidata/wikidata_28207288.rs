use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207288: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_288,
        source_type: SourceType::Wikidata,
        name: "Slow-scan television",
        extensions: &["hrz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
