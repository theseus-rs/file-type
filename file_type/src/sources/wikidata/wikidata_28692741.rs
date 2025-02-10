use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28692741: FileType = FileType {
    file_format: &FileFormat {
        id: 28_692_741,
        source_type: SourceType::Wikidata,
        name: "FAV File Format",
        extensions: &["fav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
