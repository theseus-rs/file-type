use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111342104: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_104,
        source_type: SourceType::Wikidata,
        name: "SoundStage sound info file",
        extensions: &["sfi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
