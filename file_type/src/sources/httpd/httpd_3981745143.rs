use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3981745143: FileType = FileType {
    file_format: &FileFormat {
        id: 3_981_745_143,
        source_type: SourceType::Httpd,
        name: "webturbo",
        extensions: &["wtb"],
        media_types: &["application/vnd.webturbo"],
        signatures: &[],
        related_formats: &[],
    },
};
