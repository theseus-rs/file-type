use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122404284: FileType = FileType {
    file_format: &FileFormat {
        id: 122_404_284,
        source_type: SourceType::Wikidata,
        name: "Pilot Resource File",
        extensions: &["plr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
