use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919154: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_154,
        source_type: SourceType::Wikidata,
        name: "Rhino 3D Model Backup",
        extensions: &["3dmbak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
