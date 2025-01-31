use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853182: FileFormat = FileFormat {
    id: 105_853_182,
    puid: "wikidata/105853182",
    name: "Sacred 2 save game",
    extensions: &["sacred2save"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x45, 0x58, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
