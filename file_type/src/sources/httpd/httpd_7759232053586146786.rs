use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7759232053586146786: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dts",
    extensions: &["dts"],
    media_types: &["audio/vnd.dts"],
    internal_signatures: &[],
    related_formats: &[],
};
