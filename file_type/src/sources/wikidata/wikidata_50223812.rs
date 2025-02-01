use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50223812: FileFormat = FileFormat {
    id: 50_223_812,
    puid: "wikidata/50223812",
    name: "Bluetooth Snoop Packet Capture",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
