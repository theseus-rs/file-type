use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_812449347: FileFormat = FileFormat {
    id: 812_449_347,
    source_type: SourceType::Httpd,
    name: "yamaha openscoreformat osfpvg xml",
    extensions: &["osfpvg"],
    media_types: &["application/vnd.yamaha.openscoreformat.osfpvg+xml"],
    signatures: &[],
    related_formats: &[],
};
