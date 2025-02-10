use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28009496: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_496,
        source_type: SourceType::Wikidata,
        name: "Zelda Solarus DX saved game",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
