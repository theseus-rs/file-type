use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_341496207: FileType = FileType {
    file_format: &FileFormat {
        id: 341_496_207,
        source_type: SourceType::Iana,
        name: "ecmascript (OBSOLETED in favor of text/javascript)",
        extensions: &[],
        media_types: &["application/ecmascript"],
        signatures: &[],
        related_formats: &[],
    },
};
