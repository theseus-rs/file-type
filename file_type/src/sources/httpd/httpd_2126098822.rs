use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2126098822: FileType = FileType {
    file_format: &FileFormat {
        id: 2_126_098_822,
        source_type: SourceType::Httpd,
        name: "omdoc xml",
        extensions: &["omdoc"],
        media_types: &["application/omdoc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
