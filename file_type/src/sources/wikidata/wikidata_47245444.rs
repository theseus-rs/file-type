use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47245444: FileFormat = FileFormat {
    id: 47_245_444,
    source_type: SourceType::Wikidata,
    name: "Microsoft Network Monitor Packet Capture",
    extensions: &["cap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
