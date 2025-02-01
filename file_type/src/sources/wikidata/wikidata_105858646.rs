use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858646: FileFormat = FileFormat {
    id: 105_858_646,
    puid: "wikidata/105858646",
    name: "Boundary Scan Description Language",
    extensions: &["bsdl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x65, 0x6E, 0x74, 0x69, 0x74, 0x79, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
