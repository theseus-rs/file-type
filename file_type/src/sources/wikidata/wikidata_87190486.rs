use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87190486: FileType = FileType {
    file_format: &FileFormat {
        id: 87_190_486,
        source_type: SourceType::Wikidata,
        name: "X3D 3.0",
        extensions: &["x3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
