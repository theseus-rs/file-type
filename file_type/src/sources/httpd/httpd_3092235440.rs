use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3092235440: FileType = FileType {
    file_format: &FileFormat {
        id: 3_092_235_440,
        source_type: SourceType::Httpd,
        name: "ssdl xml",
        extensions: &["ssdl"],
        media_types: &["application/ssdl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
