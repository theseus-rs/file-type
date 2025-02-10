use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_75536482: FileType = FileType {
    file_format: &FileFormat {
        id: 75_536_482,
        source_type: SourceType::Wikidata,
        name: "Ulead Photo Express image",
        extensions: &["upx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
