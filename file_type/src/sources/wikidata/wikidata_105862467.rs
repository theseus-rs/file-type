use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862467: FileFormat = FileFormat {
    id: 105_862_467,
    source_type: SourceType::Wikidata,
    name: "StarTrekker 4-channel module",
    extensions: &["mod"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4C, 0x54, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
