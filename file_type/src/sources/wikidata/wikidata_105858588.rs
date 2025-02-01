use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858588: FileFormat = FileFormat {
    id: 105_858_588,
    puid: "wikidata/105858588",
    name: "PM XV bitmap",
    extensions: &["pm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x49, 0x45, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
