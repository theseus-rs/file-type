use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3004210508: FileType = FileType {
    file_format: &FileFormat {
        id: 3_004_210_508,
        source_type: SourceType::Iana,
        name: "ecmascript (OBSOLETED in favor of text/javascript)",
        extensions: &[],
        media_types: &["text/ecmascript"],
        signatures: &[],
        related_formats: &[],
    },
};
