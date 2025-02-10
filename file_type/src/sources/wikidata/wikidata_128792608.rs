use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128792608: FileType = FileType {
    file_format: &FileFormat {
        id: 128_792_608,
        source_type: SourceType::Wikidata,
        name: "DAX formula file",
        extensions: &["dax"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
