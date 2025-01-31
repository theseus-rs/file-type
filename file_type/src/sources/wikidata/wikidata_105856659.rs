use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856659: FileFormat = FileFormat {
    id: 105_856_659,
    puid: "wikidata/105856659",
    name: "Unreal Music",
    extensions: &["umx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC1, 0x83, 0x2A, 0x9E])],
            },
        }],
    }],
    related_formats: &[],
};
