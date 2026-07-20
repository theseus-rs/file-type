use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_902758801: FileType = FileType {
    file_format: &FileFormat {
        id: 902_758_801,
        source_type: SourceType::Iana,
        name: "vnd.hekaya",
        extensions: &[],
        media_types: &["text/vnd.hekaya"],
        signatures: &[],
        related_formats: &[],
    },
};
