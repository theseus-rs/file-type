use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111519484: FileType = FileType {
    file_format: &FileFormat {
        id: 111_519_484,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcInfo Grid .nit File",
        extensions: &["nit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
