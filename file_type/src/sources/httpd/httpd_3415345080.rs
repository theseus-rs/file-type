use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3415345080: FileType = FileType {
    file_format: &FileFormat {
        id: 3_415_345_080,
        source_type: SourceType::Httpd,
        name: "silverlight app",
        extensions: &["xap"],
        media_types: &["application/x-silverlight-app"],
        signatures: &[],
        related_formats: &[],
    },
};
