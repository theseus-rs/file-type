use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1222670581: FileType = FileType {
    file_format: &FileFormat {
        id: 1_222_670_581,
        source_type: SourceType::Httpd,
        name: "dece sd",
        extensions: &["uvs", "uvvs"],
        media_types: &["video/vnd.dece.sd"],
        signatures: &[],
        related_formats: &[],
    },
};
