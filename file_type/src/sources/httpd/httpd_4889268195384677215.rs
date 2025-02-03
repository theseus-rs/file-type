use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4889268195384677215: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkix attr cert",
    extensions: &["ac"],
    media_types: &["application/pkix-attr-cert"],
    internal_signatures: &[],
    related_formats: &[],
};
