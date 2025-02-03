use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1727175336: FileFormat = FileFormat {
    id: 1_727_175_336,
    source_type: SourceType::Httpd,
    name: "dts hd",
    extensions: &["dtshd"],
    media_types: &["audio/vnd.dts.hd"],
    internal_signatures: &[],
    related_formats: &[],
};
