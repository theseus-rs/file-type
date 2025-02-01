use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852846: FileFormat = FileFormat {
    id: 105_852_846,
    puid: "wikidata/105852846",
    name: "SQR script",
    extensions: &["sqr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
