use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124856858: FileType = FileType {
    file_format: &FileFormat {
        id: 124_856_858,
        source_type: SourceType::Wikidata,
        name: "OpenFOAM Mesh file",
        extensions: &["msh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
