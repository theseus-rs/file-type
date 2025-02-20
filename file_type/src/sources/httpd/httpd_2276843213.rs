use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2276843213: FileType = FileType {
    file_format: &FileFormat {
        id: 2_276_843_213,
        source_type: SourceType::Httpd,
        name: "authorware bin",
        extensions: &["aab", "x32", "u32", "vox"],
        media_types: &["application/x-authorware-bin"],
        signatures: &[],
        related_formats: &[],
    },
};
