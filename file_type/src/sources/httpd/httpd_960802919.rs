use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_960802919: FileFormat = FileFormat {
    id: 960_802_919,
    source_type: SourceType::Httpd,
    name: "xara",
    extensions: &["xar"],
    media_types: &["application/vnd.xara"],
    signatures: &[],
    related_formats: &[],
};
