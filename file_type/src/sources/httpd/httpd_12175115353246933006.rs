use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12175115353246933006: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "subrip",
    extensions: &["srt"],
    media_types: &["application/x-subrip"],
    internal_signatures: &[],
    related_formats: &[],
};
