use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60558754: FileType = FileType {
    file_format: &FileFormat {
        id: 60_558_754,
        source_type: SourceType::Wikidata,
        name: "NuFile Exchange Archival Library",
        extensions: &["bxy", "sdk", "shk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
