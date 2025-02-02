use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8243032856616937357: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "anser web funds transfer initiation",
    extensions: &["fti"],
    media_types: &["application/vnd.anser-web-funds-transfer-initiation"],
    internal_signatures: &[],
    related_formats: &[],
};
