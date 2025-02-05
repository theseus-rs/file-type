use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_213517500: FileFormat = FileFormat {
    id: 213_517_500,
    source_type: SourceType::Httpd,
    name: "mophun certificate",
    extensions: &["mpc"],
    media_types: &["application/vnd.mophun.certificate"],
    signatures: &[],
    related_formats: &[],
};
