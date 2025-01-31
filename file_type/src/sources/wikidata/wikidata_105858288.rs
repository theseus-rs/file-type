use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858288: FileFormat = FileFormat {
    id: 105_858_288,
    puid: "wikidata/105858288",
    name: "MetaQuotes Language 4 compiled program",
    extensions: &["ex4"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x58, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
