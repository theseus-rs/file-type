use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858696: FileFormat = FileFormat {
    id: 105_858_696,
    puid: "wikidata/105858696",
    name: "Crack Art bitmap (hi-res)",
    extensions: &["ca3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x01, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
