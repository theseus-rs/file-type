use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3024262940: FileType = FileType {
    file_format: &FileFormat {
        id: 3_024_262_940,
        source_type: SourceType::Iana,
        name: "vnd.geoplan",
        extensions: &[],
        media_types: &["application/vnd.geoplan"],
        signatures: &[],
        related_formats: &[],
    },
};
