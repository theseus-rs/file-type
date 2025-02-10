use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_18812775: FileType = FileType {
    file_format: &FileFormat {
        id: 18_812_775,
        source_type: SourceType::Wikidata,
        name: "VTK format",
        extensions: &["vtk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
