use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858538: FileFormat = FileFormat {
    id: 105_858_538,
    puid: "wikidata/105858538",
    name: "PIK bitmap",
    extensions: &["pik"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD7, 0x4C, 0x4D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
