use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2655630561: FileType = FileType {
    file_format: &FileFormat {
        id: 2_655_630_561,
        source_type: SourceType::Iana,
        name: "vnd.syncml.dmtnds+wbxml",
        extensions: &[],
        media_types: &["application/vnd.syncml.dmtnds+wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
