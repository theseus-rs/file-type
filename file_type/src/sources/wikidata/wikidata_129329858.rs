use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129329858: FileType = FileType {
    file_format: &FileFormat {
        id: 129_329_858,
        source_type: SourceType::Wikidata,
        name: "Genshi file",
        extensions: &["kid"],
        media_types: &["application/x-genshi", "application/x-kid"],
        signatures: &[],
        related_formats: &[],
    },
};
