use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122411453: FileType = FileType {
    file_format: &FileFormat {
        id: 122_411_453,
        source_type: SourceType::Wikidata,
        name: "Palm Pilot Symbol File",
        extensions: &["psym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
