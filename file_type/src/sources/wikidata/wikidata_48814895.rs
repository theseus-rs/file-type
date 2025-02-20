use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48814895: FileType = FileType {
    file_format: &FileFormat {
        id: 48_814_895,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcView Project",
        extensions: &["apr"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
