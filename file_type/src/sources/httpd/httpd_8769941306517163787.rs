use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8769941306517163787: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fdsn seed",
    extensions: &["seed", "dataless"],
    media_types: &["application/vnd.fdsn.seed"],
    internal_signatures: &[],
    related_formats: &[],
};
