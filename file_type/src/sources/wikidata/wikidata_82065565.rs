use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_82065565: FileType = FileType {
    file_format: &FileFormat {
        id: 82_065_565,
        source_type: SourceType::Wikidata,
        name: "PhotoShop Extended Digital Book",
        extensions: &["edb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
