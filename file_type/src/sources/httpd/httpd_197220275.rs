use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_197220275: FileFormat = FileFormat {
    id: 197_220_275,
    source_type: SourceType::Httpd,
    name: "android package archive",
    extensions: &["apk"],
    media_types: &["application/vnd.android.package-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
