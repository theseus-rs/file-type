use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28756039: FileType = FileType {
    file_format: &FileFormat {
        id: 28_756_039,
        source_type: SourceType::Wikidata,
        name: "FLA",
        extensions: &["fla"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
