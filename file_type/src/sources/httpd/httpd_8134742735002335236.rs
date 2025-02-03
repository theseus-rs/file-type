use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8134742735002335236: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rig cryptonote",
    extensions: &["cryptonote"],
    media_types: &["application/vnd.rig.cryptonote"],
    internal_signatures: &[],
    related_formats: &[],
};
