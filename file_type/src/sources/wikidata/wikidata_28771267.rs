use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28771267: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_267,
        source_type: SourceType::Wikidata,
        name: "MLM",
        extensions: &["mlm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
