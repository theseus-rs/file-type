use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113584996: FileType = FileType {
    file_format: &FileFormat {
        id: 113_584_996,
        source_type: SourceType::Wikidata,
        name: "VideoFactory Project File",
        extensions: &["vf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
