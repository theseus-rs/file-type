use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_77905493598287429: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms works",
    extensions: &["wps", "wks", "wcm", "wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
