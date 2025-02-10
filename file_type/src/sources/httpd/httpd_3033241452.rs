use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3033241452: FileType = FileType {
    file_format: &FileFormat {
        id: 3_033_241_452,
        source_type: SourceType::Httpd,
        name: "director",
        extensions: &[
            "dir", "dcr", "dxr", "cst", "cct", "cxt", "w3d", "fgd", "swa",
        ],
        media_types: &["application/x-director"],
        signatures: &[],
        related_formats: &[],
    },
};
