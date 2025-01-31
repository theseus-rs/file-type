use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858492: FileFormat = FileFormat {
    id: 105_858_492,
    puid: "wikidata/105858492",
    name: "Cybiko Cybook eBook",
    extensions: &["book"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x79, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
