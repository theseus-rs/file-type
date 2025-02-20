use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_942105561: FileType = FileType {
    file_format: &FileFormat {
        id: 942_105_561,
        source_type: SourceType::Httpd,
        name: "rpki manifest",
        extensions: &["mft"],
        media_types: &["application/rpki-manifest"],
        signatures: &[],
        related_formats: &[],
    },
};
