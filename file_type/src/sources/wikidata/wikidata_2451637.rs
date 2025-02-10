use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2451637: FileType = FileType {
    file_format: &FileFormat {
        id: 2_451_637,
        source_type: SourceType::Wikidata,
        name: "torrent file",
        extensions: &["torrent"],
        media_types: &["application/x-bittorrent"],
        signatures: &[],
        related_formats: &[],
    },
};
