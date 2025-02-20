use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29905141: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_141,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System utility file",
        extensions: &["sas7butl", "su2", "su7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
