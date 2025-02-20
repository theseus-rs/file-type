use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_711868225: FileType = FileType {
    file_format: &FileFormat {
        id: 711_868_225,
        source_type: SourceType::Iana,
        name: "H264-RCDO",
        extensions: &[],
        media_types: &["video/H264-RCDO"],
        signatures: &[],
        related_formats: &[],
    },
};
