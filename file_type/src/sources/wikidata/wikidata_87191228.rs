use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87191228: FileType = FileType {
    file_format: &FileFormat {
        id: 87_191_228,
        source_type: SourceType::Wikidata,
        name: "X3D 3.2",
        extensions: &["x3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
