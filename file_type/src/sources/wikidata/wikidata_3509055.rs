use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3509055: FileType = FileType {
    file_format: &FileFormat {
        id: 3_509_055,
        source_type: SourceType::Wikidata,
        name: "STEP file",
        extensions: &["p21", "step", "stp"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
