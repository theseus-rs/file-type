use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1320060169: FileType = FileType {
    file_format: &FileFormat {
        id: 1_320_060_169,
        source_type: SourceType::Httpd,
        name: "bittorrent",
        extensions: &["torrent"],
        media_types: &["application/x-bittorrent"],
        signatures: &[],
        related_formats: &[],
    },
};
