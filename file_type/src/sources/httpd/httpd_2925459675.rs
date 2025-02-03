use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2925459675: FileFormat = FileFormat {
    id: 2_925_459_675,
    source_type: SourceType::Httpd,
    name: "pkix attr cert",
    extensions: &["ac"],
    media_types: &["application/pkix-attr-cert"],
    internal_signatures: &[],
    related_formats: &[],
};
