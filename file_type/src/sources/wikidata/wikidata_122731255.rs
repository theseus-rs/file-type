use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122731255: FileType = FileType {
    file_format: &FileFormat {
        id: 122_731_255,
        source_type: SourceType::Wikidata,
        name: "NCR G4 file format",
        extensions: &["ncr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
