use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1505034774: FileFormat = FileFormat {
    id: 1_505_034_774,
    source_type: SourceType::Httpd,
    name: "micrografx flo",
    extensions: &["flo"],
    media_types: &["application/vnd.micrografx.flo"],
    internal_signatures: &[],
    related_formats: &[],
};
