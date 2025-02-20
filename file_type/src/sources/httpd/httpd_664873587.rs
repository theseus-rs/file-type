use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_664873587: FileType = FileType {
    file_format: &FileFormat {
        id: 664_873_587,
        source_type: SourceType::Httpd,
        name: "x3d vrml",
        extensions: &["x3dv", "x3dvz"],
        media_types: &["model/x3d+vrml"],
        signatures: &[],
        related_formats: &[],
    },
};
