use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4115982219: FileFormat = FileFormat {
    id: 4_115_982_219,
    source_type: SourceType::Httpd,
    name: "ibm minipay",
    extensions: &["mpy"],
    media_types: &["application/vnd.ibm.minipay"],
    internal_signatures: &[],
    related_formats: &[],
};
