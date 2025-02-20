use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47483338: FileType = FileType {
    file_format: &FileFormat {
        id: 47_483_338,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System data set (Unix)",
        extensions: &["sas7bdat", "sd2", "sd7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
