use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87190680: FileType = FileType {
    file_format: &FileFormat {
        id: 87_190_680,
        source_type: SourceType::Wikidata,
        name: "X3D 3.1",
        extensions: &["x3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
