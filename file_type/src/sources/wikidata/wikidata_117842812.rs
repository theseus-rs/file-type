use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117842812: FileType = FileType {
    file_format: &FileFormat {
        id: 117_842_812,
        source_type: SourceType::Wikidata,
        name: "EDMICS 5",
        extensions: &["ed5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
