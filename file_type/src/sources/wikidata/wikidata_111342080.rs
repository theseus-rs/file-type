use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111342080: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_080,
        source_type: SourceType::Wikidata,
        name: "SoundStage sound data file",
        extensions: &["sfd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
