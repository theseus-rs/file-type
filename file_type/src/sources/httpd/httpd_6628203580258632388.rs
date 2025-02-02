use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6628203580258632388: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "authorware bin",
    extensions: &["aab", "x32", "u32", "vox"],
    media_types: &["application/x-authorware-bin"],
    internal_signatures: &[],
    related_formats: &[],
};
