use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47483371: FileType = FileType {
    file_format: &FileFormat {
        id: 47_483_371,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System Data (Windows)",
        extensions: &["sas7bdat", "sd7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
