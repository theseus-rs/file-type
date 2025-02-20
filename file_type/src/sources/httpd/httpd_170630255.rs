use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_170630255: FileType = FileType {
    file_format: &FileFormat {
        id: 170_630_255,
        source_type: SourceType::Httpd,
        name: "3gpp pic bw var",
        extensions: &["pvb"],
        media_types: &["application/vnd.3gpp.pic-bw-var"],
        signatures: &[],
        related_formats: &[],
    },
};
