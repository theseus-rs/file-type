use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113663059: FileType = FileType {
    file_format: &FileFormat {
        id: 113_663_059,
        source_type: SourceType::Wikidata,
        name: "Coordinate 3D",
        extensions: &["c3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
