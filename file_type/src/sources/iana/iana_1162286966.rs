use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1162286966: FileType = FileType {
    file_format: &FileFormat {
        id: 1_162_286_966,
        source_type: SourceType::Iana,
        name: "vnd.geogebra.pinboard",
        extensions: &[],
        media_types: &["application/vnd.geogebra.pinboard"],
        signatures: &[],
        related_formats: &[],
    },
};
