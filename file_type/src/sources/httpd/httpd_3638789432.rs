use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3638789432: FileFormat = FileFormat {
    id: 3_638_789_432,
    source_type: SourceType::Httpd,
    name: "anser web funds transfer initiation",
    extensions: &["fti"],
    media_types: &["application/vnd.anser-web-funds-transfer-initiation"],
    signatures: &[],
    related_formats: &[],
};
