use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3159007645: FileType = FileType {
    file_format: &FileFormat {
        id: 3_159_007_645,
        source_type: SourceType::Httpd,
        name: "sun xml draw",
        extensions: &["sxd"],
        media_types: &["application/vnd.sun.xml.draw"],
        signatures: &[],
        related_formats: &[],
    },
};
