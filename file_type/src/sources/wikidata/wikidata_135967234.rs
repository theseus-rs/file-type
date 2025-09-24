use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135967234: FileType = FileType {
    file_format: &FileFormat {
        id: 135_967_234,
        source_type: SourceType::Wikidata,
        name: "PLOT3D XYZ file",
        extensions: &["xyz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
