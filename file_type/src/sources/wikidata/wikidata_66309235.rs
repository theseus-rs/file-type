use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66309235: FileType = FileType {
    file_format: &FileFormat {
        id: 66_309_235,
        source_type: SourceType::Wikidata,
        name: "Access Blank Project Template",
        extensions: &["adn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
