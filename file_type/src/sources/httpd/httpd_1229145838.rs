use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1229145838: FileType = FileType {
    file_format: &FileFormat {
        id: 1_229_145_838,
        source_type: SourceType::Httpd,
        name: "amiga ami",
        extensions: &["ami"],
        media_types: &["application/vnd.amiga.ami"],
        signatures: &[],
        related_formats: &[],
    },
};
