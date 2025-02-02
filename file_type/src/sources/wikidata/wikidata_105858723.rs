use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858723: FileFormat = FileFormat {
    id: 105_858_723,
    source_type: SourceType::Wikidata,
    name: "Sequencer One Block",
    extensions: &["blk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x6C, 0x6F, 0x6B, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
