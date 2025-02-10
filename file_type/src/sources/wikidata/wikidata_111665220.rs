use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111665220: FileType = FileType {
    file_format: &FileFormat {
        id: 111_665_220,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Text Template",
        extensions: &["ott"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
