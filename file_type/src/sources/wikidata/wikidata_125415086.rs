use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125415086: FileType = FileType {
    file_format: &FileFormat {
        id: 125_415_086,
        source_type: SourceType::Wikidata,
        name: "TCM document",
        extensions: &["tcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
