use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967125: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_125,
        source_type: SourceType::Wikidata,
        name: "CMC",
        extensions: &["cmc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
