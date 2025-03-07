use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132818449: FileType = FileType {
    file_format: &FileFormat {
        id: 132_818_449,
        source_type: SourceType::Wikidata,
        name: "Visualization Toolkit 4.2 file",
        extensions: &["vtk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
