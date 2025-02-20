use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71276559: FileType = FileType {
    file_format: &FileFormat {
        id: 71_276_559,
        source_type: SourceType::Wikidata,
        name: "PESIM file format",
        extensions: &["pes"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
