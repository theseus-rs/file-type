use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2191419818: FileType = FileType {
    file_format: &FileFormat {
        id: 2_191_419_818,
        source_type: SourceType::Httpd,
        name: "sun xml writer global",
        extensions: &["sxg"],
        media_types: &["application/vnd.sun.xml.writer.global"],
        signatures: &[],
        related_formats: &[],
    },
};
