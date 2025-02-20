use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121092987: FileType = FileType {
    file_format: &FileFormat {
        id: 121_092_987,
        source_type: SourceType::Wikidata,
        name: "Punch! 3D Object",
        extensions: &["pob"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
