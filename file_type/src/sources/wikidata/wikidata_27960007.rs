use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960007: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_007,
        source_type: SourceType::Wikidata,
        name: "RK Audio",
        extensions: &["rka"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
