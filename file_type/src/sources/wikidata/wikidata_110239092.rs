use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110239092: FileType = FileType {
    file_format: &FileFormat {
        id: 110_239_092,
        source_type: SourceType::Wikidata,
        name: "Avid Editor Format",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
