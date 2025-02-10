use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3415200755: FileType = FileType {
    file_format: &FileFormat {
        id: 3_415_200_755,
        source_type: SourceType::Iana,
        name: "vnd.mapbox-vector-tile",
        extensions: &[],
        media_types: &["application/vnd.mapbox-vector-tile"],
        signatures: &[],
        related_formats: &[],
    },
};
