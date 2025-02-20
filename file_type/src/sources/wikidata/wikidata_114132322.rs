use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114132322: FileType = FileType {
    file_format: &FileFormat {
        id: 114_132_322,
        source_type: SourceType::Wikidata,
        name: "Chem3D Cartesian Coordinates 2",
        extensions: &["cc2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
