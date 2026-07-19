use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138747602: FileType = FileType {
    file_format: &FileFormat {
        id: 138_747_602,
        source_type: SourceType::Wikidata,
        name: "Fusion 360 CAM Data file",
        extensions: &["cam360"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
