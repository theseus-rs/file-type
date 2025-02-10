use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50223857: FileType = FileType {
    file_format: &FileFormat {
        id: 50_223_857,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcMap Document",
        extensions: &["mxd"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
