use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_18812775: FileType = FileType {
    file_format: &FileFormat {
        id: 18_812_775,
        source_type: SourceType::Wikidata,
        name: "VTK file format family",
        extensions: &["vtk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
