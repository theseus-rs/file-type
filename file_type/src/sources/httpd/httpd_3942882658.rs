use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3942882658: FileFormat = FileFormat {
    id: 3_942_882_658,
    source_type: SourceType::Httpd,
    name: "prs lines tag",
    extensions: &["dsc"],
    media_types: &["text/prs.lines.tag"],
    internal_signatures: &[],
    related_formats: &[],
};
