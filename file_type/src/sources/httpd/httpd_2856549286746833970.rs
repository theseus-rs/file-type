use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2856549286746833970: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "vcard",
    extensions: &["vcf"],
    media_types: &["text/x-vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
