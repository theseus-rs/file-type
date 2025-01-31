use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859020: FileFormat = FileFormat {
    id: 105_859_020,
    puid: "wikidata/105859020",
    name: "Personal Paint encrypted bitmap",
    extensions: &["pic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x52, 0x59, 0x50, 0x54, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
