use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1425699405: FileType = FileType {
    file_format: &FileFormat {
        id: 1_425_699_405,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-dialog-speech+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-dialog-speech+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
