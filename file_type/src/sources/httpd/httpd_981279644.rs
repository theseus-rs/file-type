use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_981279644: FileFormat = FileFormat {
    id: 981_279_644,
    source_type: SourceType::Httpd,
    name: "dvb ait",
    extensions: &["ait"],
    media_types: &["application/vnd.dvb.ait"],
    internal_signatures: &[],
    related_formats: &[],
};
