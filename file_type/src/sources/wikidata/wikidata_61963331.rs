use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61963331: FileType = FileType {
    file_format: &FileFormat {
        id: 61_963_331,
        source_type: SourceType::Wikidata,
        name: "pulse EKKO data file",
        extensions: &["dt1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
