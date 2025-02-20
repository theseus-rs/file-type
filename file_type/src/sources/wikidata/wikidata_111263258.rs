use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111263258: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_258,
        source_type: SourceType::Wikidata,
        name: "Soundcap/SoundEdit instrument",
        extensions: &["dewf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
