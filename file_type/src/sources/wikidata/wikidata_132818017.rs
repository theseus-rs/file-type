use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132818017: FileType = FileType {
    file_format: &FileFormat {
        id: 132_818_017,
        source_type: SourceType::Wikidata,
        name: "Visualization Toolkit 1.0 file",
        extensions: &["vtk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
