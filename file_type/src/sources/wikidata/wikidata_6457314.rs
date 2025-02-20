use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6457314: FileType = FileType {
    file_format: &FileFormat {
        id: 6_457_314,
        source_type: SourceType::Wikidata,
        name: "LBR",
        extensions: &["lbr", "lqr", "lyr", "lzr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
