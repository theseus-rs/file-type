use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125253619: FileType = FileType {
    file_format: &FileFormat {
        id: 125_253_619,
        source_type: SourceType::Wikidata,
        name: "Simple interaction file",
        extensions: &["sif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
