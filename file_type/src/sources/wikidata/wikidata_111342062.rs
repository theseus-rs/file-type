use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111342062: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_062,
        source_type: SourceType::Wikidata,
        name: "Melody Machine compressed SoundFont",
        extensions: &["sfark"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
