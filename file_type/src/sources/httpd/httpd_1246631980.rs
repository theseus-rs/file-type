use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1246631980: FileFormat = FileFormat {
    id: 1_246_631_980,
    source_type: SourceType::Httpd,
    name: "fdsn seed",
    extensions: &["seed", "dataless"],
    media_types: &["application/vnd.fdsn.seed"],
    signatures: &[],
    related_formats: &[],
};
