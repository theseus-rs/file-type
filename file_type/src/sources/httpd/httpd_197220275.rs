use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_197220275: FileType = FileType {
    file_format: &FileFormat {
        id: 197_220_275,
        source_type: SourceType::Httpd,
        name: "android package archive",
        extensions: &["apk"],
        media_types: &["application/vnd.android.package-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
