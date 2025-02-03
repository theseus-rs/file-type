use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855900: FileFormat = FileFormat {
    id: 105_855_900,
    source_type: SourceType::Wikidata,
    name: "Seal registry Data",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x52, 0x45, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
