use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7903498: FileType = FileType {
    file_format: &FileFormat {
        id: 7_903_498,
        source_type: SourceType::Wikidata,
        name: "UTZ",
        extensions: &["utz"],
        media_types: &["application/vnd.uiq.theme"],
        signatures: &[],
        related_formats: &[],
    },
};
