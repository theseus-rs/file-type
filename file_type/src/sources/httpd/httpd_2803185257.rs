use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2803185257: FileType = FileType {
    file_format: &FileFormat {
        id: 2_803_185_257,
        source_type: SourceType::Httpd,
        name: "cdmi queue",
        extensions: &["cdmiq"],
        media_types: &["application/cdmi-queue"],
        signatures: &[],
        related_formats: &[],
    },
};
