use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_251540190: FileType = FileType {
    file_format: &FileFormat {
        id: 251_540_190,
        source_type: SourceType::Httpd,
        name: "intercon formnet",
        extensions: &["xpw", "xpx"],
        media_types: &["application/vnd.intercon.formnet"],
        signatures: &[],
        related_formats: &[],
    },
};
