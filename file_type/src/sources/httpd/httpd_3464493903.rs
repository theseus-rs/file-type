use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3464493903: FileType = FileType {
    file_format: &FileFormat {
        id: 3_464_493_903,
        source_type: SourceType::Httpd,
        name: "msbinder",
        extensions: &["obd"],
        media_types: &["application/x-msbinder"],
        signatures: &[],
        related_formats: &[],
    },
};
