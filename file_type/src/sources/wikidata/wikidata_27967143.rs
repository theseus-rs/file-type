use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967143: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_143,
        source_type: SourceType::Wikidata,
        name: "DigiTrekker module",
        extensions: &["dtm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
