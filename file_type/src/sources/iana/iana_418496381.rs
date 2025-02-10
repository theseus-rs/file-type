use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_418496381: FileType = FileType {
    file_format: &FileFormat {
        id: 418_496_381,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.signal",
        extensions: &[],
        media_types: &["application/vnd.uplanet.signal"],
        signatures: &[],
        related_formats: &[],
    },
};
