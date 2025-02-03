use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2272512056: FileFormat = FileFormat {
    id: 2_272_512_056,
    source_type: SourceType::Httpd,
    name: "hyperstudio",
    extensions: &["stk"],
    media_types: &["application/hyperstudio"],
    internal_signatures: &[],
    related_formats: &[],
};
