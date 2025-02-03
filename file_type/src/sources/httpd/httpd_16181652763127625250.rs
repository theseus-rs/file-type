use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16181652763127625250: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "vcx",
    extensions: &["vcx"],
    media_types: &["application/vnd.vcx"],
    internal_signatures: &[],
    related_formats: &[],
};
