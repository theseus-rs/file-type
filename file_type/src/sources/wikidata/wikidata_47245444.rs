use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47245444: FileFormat = FileFormat {
    id: 47_245_444,
    source_type: SourceType::Wikidata,
    name: "Microsoft Network Monitor Packet Capture",
    extensions: &["cap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
