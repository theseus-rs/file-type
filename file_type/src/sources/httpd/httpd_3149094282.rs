use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3149094282: FileType = FileType {
    file_format: &FileFormat {
        id: 3_149_094_282,
        source_type: SourceType::Httpd,
        name: "pn realaudio",
        extensions: &["ram", "ra"],
        media_types: &["audio/x-pn-realaudio"],
        signatures: &[],
        related_formats: &[],
    },
};
