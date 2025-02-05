use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3736026393: FileFormat = FileFormat {
    id: 3_736_026_393,
    source_type: SourceType::Httpd,
    name: "xpinstall",
    extensions: &["xpi"],
    media_types: &["application/x-xpinstall"],
    signatures: &[],
    related_formats: &[],
};
