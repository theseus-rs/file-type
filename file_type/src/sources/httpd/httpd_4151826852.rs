use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4151826852: FileFormat = FileFormat {
    id: 4_151_826_852,
    source_type: SourceType::Httpd,
    name: "quark quarkxpress",
    extensions: &["qxd", "qxt", "qwd", "qwt", "qxl", "qxb"],
    media_types: &["application/vnd.quark.quarkxpress"],
    signatures: &[],
    related_formats: &[],
};
