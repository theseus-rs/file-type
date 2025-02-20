use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121543029: FileType = FileType {
    file_format: &FileFormat {
        id: 121_543_029,
        source_type: SourceType::Wikidata,
        name: "DeductionPro 2008 Data file",
        extensions: &["d08"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
