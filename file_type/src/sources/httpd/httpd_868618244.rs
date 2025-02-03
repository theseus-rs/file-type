use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_868618244: FileFormat = FileFormat {
    id: 868_618_244,
    source_type: SourceType::Httpd,
    name: "ahead space",
    extensions: &["ahead"],
    media_types: &["application/vnd.ahead.space"],
    internal_signatures: &[],
    related_formats: &[],
};
