use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_96000078: FileType = FileType {
    file_format: &FileFormat {
        id: 96_000_078,
        source_type: SourceType::Wikidata,
        name: "NOFF 3D geometry format",
        extensions: &["noff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
