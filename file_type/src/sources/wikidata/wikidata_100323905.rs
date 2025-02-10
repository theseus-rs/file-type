use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100323905: FileType = FileType {
    file_format: &FileFormat {
        id: 100_323_905,
        source_type: SourceType::Wikidata,
        name: "PFS:Write Document",
        extensions: &["pfs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
