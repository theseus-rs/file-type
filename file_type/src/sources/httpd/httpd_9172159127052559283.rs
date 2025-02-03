use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9172159127052559283: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ezpix album",
    extensions: &["ez2"],
    media_types: &["application/vnd.ezpix-album"],
    internal_signatures: &[],
    related_formats: &[],
};
