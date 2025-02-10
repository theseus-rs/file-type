use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100323885: FileType = FileType {
    file_format: &FileFormat {
        id: 100_323_885,
        source_type: SourceType::Wikidata,
        name: "Corel Gallery Clipart",
        extensions: &["bmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
