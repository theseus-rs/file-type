use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110239358: FileType = FileType {
    file_format: &FileFormat {
        id: 110_239_358,
        source_type: SourceType::Wikidata,
        name: "CompanyMOVE ShowPlanner",
        extensions: &["sex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
