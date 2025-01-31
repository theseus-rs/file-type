use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856178: FileFormat = FileFormat {
    id: 105_856_178,
    puid: "wikidata/105856178",
    name: "FL Studio Drum Pattern",
    extensions: &["dmptrn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
