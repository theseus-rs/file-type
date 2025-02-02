use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18202258566732886046: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "quark quarkxpress",
    extensions: &["qxd", "qxt", "qwd", "qwt", "qxl", "qxb"],
    media_types: &["application/vnd.quark.quarkxpress"],
    internal_signatures: &[],
    related_formats: &[],
};
