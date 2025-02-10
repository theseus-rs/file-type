use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50498951: FileType = FileType {
    file_format: &FileFormat {
        id: 50_498_951,
        source_type: SourceType::Wikidata,
        name: "OGR GFS File",
        extensions: &["gfs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
