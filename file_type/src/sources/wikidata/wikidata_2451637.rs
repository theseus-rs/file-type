use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2451637: FileFormat = FileFormat {
    id: 2_451_637,
    puid: "wikidata/2451637",
    name: "torrent file",
    extensions: &["torrent"],
    media_types: &["application/x-bittorrent"],
    internal_signatures: &[],
    related_formats: &[],
};
