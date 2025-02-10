use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110502531: FileType = FileType {
    file_format: &FileFormat {
        id: 110_502_531,
        source_type: SourceType::Wikidata,
        name: "ISDOCX Information System Document (Generic)",
        extensions: &["isdocx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
