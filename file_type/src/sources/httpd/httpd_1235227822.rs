use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1235227822: FileType = FileType {
    file_format: &FileFormat {
        id: 1_235_227_822,
        source_type: SourceType::Httpd,
        name: "3gpp2 tcap",
        extensions: &["tcap"],
        media_types: &["application/vnd.3gpp2.tcap"],
        signatures: &[],
        related_formats: &[],
    },
};
