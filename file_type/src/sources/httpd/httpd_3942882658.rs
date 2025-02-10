use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3942882658: FileType = FileType {
    file_format: &FileFormat {
        id: 3_942_882_658,
        source_type: SourceType::Httpd,
        name: "prs lines tag",
        extensions: &["dsc"],
        media_types: &["text/prs.lines.tag"],
        signatures: &[],
        related_formats: &[],
    },
};
