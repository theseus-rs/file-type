use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858209: FileFormat = FileFormat {
    id: 105_858_209,
    puid: "wikidata/105858209",
    name: "MetaQuotes Language 5 compiled program",
    extensions: &["ex4"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x58, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
