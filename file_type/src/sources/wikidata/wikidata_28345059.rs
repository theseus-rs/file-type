use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28345059: FileType = FileType {
    file_format: &FileFormat {
        id: 28_345_059,
        source_type: SourceType::Wikidata,
        name: "XP3",
        extensions: &["xp3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
