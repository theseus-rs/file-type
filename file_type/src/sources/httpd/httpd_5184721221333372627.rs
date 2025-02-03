use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5184721221333372627: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dts hd",
    extensions: &["dtshd"],
    media_types: &["audio/vnd.dts.hd"],
    internal_signatures: &[],
    related_formats: &[],
};
