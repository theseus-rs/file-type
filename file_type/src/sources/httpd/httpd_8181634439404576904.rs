use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8181634439404576904: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dvb file",
    extensions: &["dvb"],
    media_types: &["video/vnd.dvb.file"],
    internal_signatures: &[],
    related_formats: &[],
};
