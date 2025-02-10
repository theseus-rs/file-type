use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122170423: FileType = FileType {
    file_format: &FileFormat {
        id: 122_170_423,
        source_type: SourceType::Wikidata,
        name: "AmnezaVPN profile",
        extensions: &["vpn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
