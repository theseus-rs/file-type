use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137704964: FileType = FileType {
    file_format: &FileFormat {
        id: 137_704_964,
        source_type: SourceType::Wikidata,
        name: "Server Response File",
        extensions: &["srf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
