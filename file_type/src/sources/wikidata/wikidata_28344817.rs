use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28344817: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_817,
        source_type: SourceType::Wikidata,
        name: "Arts and Letters clip art library",
        extensions: &["yal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
