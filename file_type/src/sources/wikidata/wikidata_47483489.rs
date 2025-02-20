use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47483489: FileType = FileType {
    file_format: &FileFormat {
        id: 47_483_489,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System Data (Unix)",
        extensions: &["sas7bdat", "sd7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
