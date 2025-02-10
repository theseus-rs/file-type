use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_11715083: FileType = FileType {
    file_format: &FileFormat {
        id: 11_715_083,
        source_type: SourceType::Httpd,
        name: "visio",
        extensions: &["vsd", "vst", "vss", "vsw"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[],
    },
};
