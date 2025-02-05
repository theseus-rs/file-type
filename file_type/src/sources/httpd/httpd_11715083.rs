use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11715083: FileFormat = FileFormat {
    id: 11_715_083,
    source_type: SourceType::Httpd,
    name: "visio",
    extensions: &["vsd", "vst", "vss", "vsw"],
    media_types: &["application/vnd.visio"],
    signatures: &[],
    related_formats: &[],
};
