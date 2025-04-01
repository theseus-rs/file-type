use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133756343: FileType = FileType {
    file_format: &FileFormat {
        id: 133_756_343,
        source_type: SourceType::Wikidata,
        name: "Hires Player Missile file",
        extensions: &["hpm"],
        media_types: &["image/x-hires-player-missile"],
        signatures: &[],
        related_formats: &[],
    },
};
