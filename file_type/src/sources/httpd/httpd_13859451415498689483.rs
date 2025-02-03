use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13859451415498689483: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mp2t",
    extensions: &["ts", "m2t", "m2ts", "mts"],
    media_types: &["video/mp2t"],
    internal_signatures: &[],
    related_formats: &[],
};
