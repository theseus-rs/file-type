use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113543215: FileType = FileType {
    file_format: &FileFormat {
        id: 113_543_215,
        source_type: SourceType::Wikidata,
        name: "dBASE Windows Form File",
        extensions: &["wfm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
