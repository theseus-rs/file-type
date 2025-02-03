use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6379457667655792030: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "director",
    extensions: &[
        "dir", "dcr", "dxr", "cst", "cct", "cxt", "w3d", "fgd", "swa",
    ],
    media_types: &["application/x-director"],
    internal_signatures: &[],
    related_formats: &[],
};
