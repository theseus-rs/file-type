use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16428619282818802010: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "noblenet sealer",
    extensions: &["nns"],
    media_types: &["application/vnd.noblenet-sealer"],
    internal_signatures: &[],
    related_formats: &[],
};
