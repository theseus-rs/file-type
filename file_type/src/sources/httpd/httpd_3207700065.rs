use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3207700065: FileFormat = FileFormat {
    id: 3_207_700_065,
    source_type: SourceType::Httpd,
    name: "symbian install",
    extensions: &["sis", "sisx"],
    media_types: &["application/vnd.symbian.install"],
    signatures: &[],
    related_formats: &[],
};
