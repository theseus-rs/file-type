use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3033241452: FileFormat = FileFormat {
    id: 3_033_241_452,
    source_type: SourceType::Httpd,
    name: "director",
    extensions: &[
        "dir", "dcr", "dxr", "cst", "cct", "cxt", "w3d", "fgd", "swa",
    ],
    media_types: &["application/x-director"],
    signatures: &[],
    related_formats: &[],
};
