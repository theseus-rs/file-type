use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859959: FileFormat = FileFormat {
    id: 105_859_959,
    puid: "wikidata/105859959",
    name: "VVV Virtual Volume View database",
    extensions: &["vvv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x00, 0x39, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
