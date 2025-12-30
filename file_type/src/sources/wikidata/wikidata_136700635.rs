use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136700635: FileType = FileType {
    file_format: &FileFormat {
        id: 136_700_635,
        source_type: SourceType::Wikidata,
        name: "Nintendo GameCube Game Disc image",
        extensions: &["gcm", "iso"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
