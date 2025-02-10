use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113274726: FileType = FileType {
    file_format: &FileFormat {
        id: 113_274_726,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Deluxe Label",
        extensions: &["pda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
