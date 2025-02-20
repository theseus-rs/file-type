use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111512403: FileType = FileType {
    file_format: &FileFormat {
        id: 111_512_403,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcInfo .dat file (external)",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
