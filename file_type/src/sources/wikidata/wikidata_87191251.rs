use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87191251: FileType = FileType {
    file_format: &FileFormat {
        id: 87_191_251,
        source_type: SourceType::Wikidata,
        name: "X3D 3.3",
        extensions: &["x3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
