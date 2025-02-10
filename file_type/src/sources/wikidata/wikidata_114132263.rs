use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114132263: FileType = FileType {
    file_format: &FileFormat {
        id: 114_132_263,
        source_type: SourceType::Wikidata,
        name: "Chem3D Cartesian Coordinates 1",
        extensions: &["cc1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
