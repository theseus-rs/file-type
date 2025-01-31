use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47245444: FileFormat = FileFormat {
    id: 47_245_444,
    puid: "wikidata/47245444",
    name: "Microsoft Network Monitor Packet Capture",
    extensions: &["cap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
