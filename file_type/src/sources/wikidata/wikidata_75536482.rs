use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
