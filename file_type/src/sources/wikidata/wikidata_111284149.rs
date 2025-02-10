use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111284149: FileType = FileType {
    file_format: &FileFormat {
        id: 111_284_149,
        source_type: SourceType::Wikidata,
        name: "Bells, Whistles, Sound Boards module",
        extensions: &["gdm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
