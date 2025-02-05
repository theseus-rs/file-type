use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864128: FileFormat = FileFormat {
    id: 105_864_128,
    source_type: SourceType::Wikidata,
    name: "The Player 4.0b module",
    extensions: &["p40"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x34, 0x30, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
