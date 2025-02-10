use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_154740816: FileType = FileType {
    file_format: &FileFormat {
        id: 154_740_816,
        source_type: SourceType::Iana,
        name: "vnd.geogebra.slides",
        extensions: &[],
        media_types: &["application/vnd.geogebra.slides"],
        signatures: &[],
        related_formats: &[],
    },
};
