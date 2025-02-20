use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1185389084: FileType = FileType {
    file_format: &FileFormat {
        id: 1_185_389_084,
        source_type: SourceType::Httpd,
        name: "spotfire dxp",
        extensions: &["dxp"],
        media_types: &["application/vnd.spotfire.dxp"],
        signatures: &[],
        related_formats: &[],
    },
};
