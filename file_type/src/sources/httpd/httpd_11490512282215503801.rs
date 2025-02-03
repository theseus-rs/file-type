use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11490512282215503801: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "android package archive",
    extensions: &["apk"],
    media_types: &["application/vnd.android.package-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
