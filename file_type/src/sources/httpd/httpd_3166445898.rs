use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3166445898: FileType = FileType {
    file_format: &FileFormat {
        id: 3_166_445_898,
        source_type: SourceType::Httpd,
        name: "mesh",
        extensions: &["msh", "mesh", "silo"],
        media_types: &["model/mesh"],
        signatures: &[],
        related_formats: &[],
    },
};
