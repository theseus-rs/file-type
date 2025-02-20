use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100299731: FileType = FileType {
    file_format: &FileFormat {
        id: 100_299_731,
        source_type: SourceType::Wikidata,
        name: "Flow Charting file format, version 6",
        extensions: &["fcx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
