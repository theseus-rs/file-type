use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2758758858: FileFormat = FileFormat {
    id: 2_758_758_858,
    source_type: SourceType::Httpd,
    name: "cmx",
    extensions: &["cmx"],
    media_types: &["image/x-cmx"],
    internal_signatures: &[],
    related_formats: &[],
};
