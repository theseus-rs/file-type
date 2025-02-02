use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5012686256345022475: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dvb ait",
    extensions: &["ait"],
    media_types: &["application/vnd.dvb.ait"],
    internal_signatures: &[],
    related_formats: &[],
};
