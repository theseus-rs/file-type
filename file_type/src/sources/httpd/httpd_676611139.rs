use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_676611139: FileFormat = FileFormat {
    id: 676_611_139,
    source_type: SourceType::Httpd,
    name: "vcard",
    extensions: &["vcf"],
    media_types: &["text/x-vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
