use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_32097740: FileType = FileType {
    file_format: &FileFormat {
        id: 32_097_740,
        source_type: SourceType::Wikidata,
        name: "DAT",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
