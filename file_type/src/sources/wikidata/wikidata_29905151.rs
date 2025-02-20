use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29905151: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_151,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System permanent utility file",
        extensions: &["sas7bput", "sp2", "sp7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
