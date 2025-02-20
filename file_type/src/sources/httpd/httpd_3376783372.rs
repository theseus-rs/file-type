use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3376783372: FileType = FileType {
    file_format: &FileFormat {
        id: 3_376_783_372,
        source_type: SourceType::Httpd,
        name: "digital winds",
        extensions: &["eol"],
        media_types: &["audio/vnd.digital-winds"],
        signatures: &[],
        related_formats: &[],
    },
};
