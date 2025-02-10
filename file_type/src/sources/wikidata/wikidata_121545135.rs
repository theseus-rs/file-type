use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121545135: FileType = FileType {
    file_format: &FileFormat {
        id: 121_545_135,
        source_type: SourceType::Wikidata,
        name: "At Home 2012 Tax Return File",
        extensions: &["t12"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
