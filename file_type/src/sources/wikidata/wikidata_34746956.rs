use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34746956: FileType = FileType {
    file_format: &FileFormat {
        id: 34_746_956,
        source_type: SourceType::Wikidata,
        name: "STATISTICA In-place Database File",
        extensions: &["sti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
