use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29904543: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_543,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System data set view",
        extensions: &["sas7bvew", "sv2", "sv7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
