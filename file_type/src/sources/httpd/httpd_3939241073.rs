use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3939241073: FileFormat = FileFormat {
    id: 3_939_241_073,
    source_type: SourceType::Httpd,
    name: "lotus wordpro",
    extensions: &["lwp"],
    media_types: &["application/vnd.lotus-wordpro"],
    internal_signatures: &[],
    related_formats: &[],
};
