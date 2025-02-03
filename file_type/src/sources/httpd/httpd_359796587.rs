use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_359796587: FileFormat = FileFormat {
    id: 359_796_587,
    source_type: SourceType::Httpd,
    name: "noblenet sealer",
    extensions: &["nns"],
    media_types: &["application/vnd.noblenet-sealer"],
    internal_signatures: &[],
    related_formats: &[],
};
