use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2451637: FileFormat = FileFormat {
    id: 2_451_637,
    source_type: SourceType::Wikidata,
    name: "torrent file",
    extensions: &["torrent"],
    media_types: &["application/x-bittorrent"],
    internal_signatures: &[],
    related_formats: &[],
};
