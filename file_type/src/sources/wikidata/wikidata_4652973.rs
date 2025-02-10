use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4652973: FileType = FileType {
    file_format: &FileFormat {
        id: 4_652_973,
        source_type: SourceType::Wikidata,
        name: "ANIM",
        extensions: &["anim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
