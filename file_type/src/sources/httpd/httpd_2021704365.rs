use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2021704365: FileType = FileType {
    file_format: &FileFormat {
        id: 2_021_704_365,
        source_type: SourceType::Httpd,
        name: "smv",
        extensions: &["smv"],
        media_types: &["video/x-smv"],
        signatures: &[],
        related_formats: &[],
    },
};
