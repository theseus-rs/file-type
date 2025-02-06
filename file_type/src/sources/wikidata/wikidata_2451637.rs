use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2451637: FileFormat = FileFormat {
    id: 2_451_637,
    source_type: SourceType::Wikidata,
    name: "torrent file",
    extensions: &["torrent"],
    media_types: &["application/x-bittorrent"],
    signatures: &[],
    related_formats: &[],
};
