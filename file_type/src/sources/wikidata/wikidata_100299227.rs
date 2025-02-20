use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100299227: FileType = FileType {
    file_format: &FileFormat {
        id: 100_299_227,
        source_type: SourceType::Wikidata,
        name: "Flow Charting file format, version 5",
        extensions: &["fc5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
