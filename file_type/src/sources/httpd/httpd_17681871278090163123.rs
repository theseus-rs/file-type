use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17681871278090163123: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mobipocket ebook",
    extensions: &["prc", "mobi"],
    media_types: &["application/x-mobipocket-ebook"],
    internal_signatures: &[],
    related_formats: &[],
};
