use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8096259863664211066: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "bittorrent",
    extensions: &["torrent"],
    media_types: &["application/x-bittorrent"],
    internal_signatures: &[],
    related_formats: &[],
};
