use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55739333: FileType = FileType {
    file_format: &FileFormat {
        id: 55_739_333,
        source_type: SourceType::Wikidata,
        name: "CRAM file format, version 2",
        extensions: &["cram"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
