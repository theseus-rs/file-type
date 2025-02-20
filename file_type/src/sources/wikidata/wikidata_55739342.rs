use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55739342: FileType = FileType {
    file_format: &FileFormat {
        id: 55_739_342,
        source_type: SourceType::Wikidata,
        name: "CRAM file format, version 3",
        extensions: &["cram"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
