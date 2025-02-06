use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50223812: FileFormat = FileFormat {
    id: 50_223_812,
    source_type: SourceType::Wikidata,
    name: "Bluetooth Snoop Packet Capture",
    extensions: &["log"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
