use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3736026393: FileType = FileType {
    file_format: &FileFormat {
        id: 3_736_026_393,
        source_type: SourceType::Httpd,
        name: "xpinstall",
        extensions: &["xpi"],
        media_types: &["application/x-xpinstall"],
        signatures: &[],
        related_formats: &[],
    },
};
