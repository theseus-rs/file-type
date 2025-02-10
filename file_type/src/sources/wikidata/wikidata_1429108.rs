use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1429108: FileType = FileType {
    file_format: &FileFormat {
        id: 1_429_108,
        source_type: SourceType::Wikidata,
        name: "NZB",
        extensions: &["nzb"],
        media_types: &["application/x-nzb"],
        signatures: &[],
        related_formats: &[],
    },
};
