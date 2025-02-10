use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47483382: FileType = FileType {
    file_format: &FileFormat {
        id: 47_483_382,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System Data (Unix)",
        extensions: &["sas7bdat", "sd7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
