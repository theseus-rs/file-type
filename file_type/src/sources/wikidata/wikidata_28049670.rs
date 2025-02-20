use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28049670: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_670,
        source_type: SourceType::Wikidata,
        name: "Autodesk 3D Studio ASCII format",
        extensions: &["asc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
