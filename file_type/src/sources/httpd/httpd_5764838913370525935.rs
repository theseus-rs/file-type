use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5764838913370525935: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms pki seccat",
    extensions: &["cat"],
    media_types: &["application/vnd.ms-pki.seccat"],
    internal_signatures: &[],
    related_formats: &[],
};
