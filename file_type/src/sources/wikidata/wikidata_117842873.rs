use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117842873: FileType = FileType {
    file_format: &FileFormat {
        id: 117_842_873,
        source_type: SourceType::Wikidata,
        name: "EDMICS 6",
        extensions: &["ed6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
