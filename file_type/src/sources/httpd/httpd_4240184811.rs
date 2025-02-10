use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4240184811: FileType = FileType {
    file_format: &FileFormat {
        id: 4_240_184_811,
        source_type: SourceType::Httpd,
        name: "ms powerpoint presentation macroenabled 12",
        extensions: &["pptm"],
        media_types: &["application/vnd.ms-powerpoint.presentation.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
