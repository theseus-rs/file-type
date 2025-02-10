use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50375294: FileType = FileType {
    file_format: &FileFormat {
        id: 50_375_294,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcScene Document",
        extensions: &["sxd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
