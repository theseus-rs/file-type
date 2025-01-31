use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851320: FileFormat = FileFormat {
    id: 105_851_320,
    puid: "wikidata/105851320",
    name: "Pokemon Online team",
    extensions: &["tp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x54, 0x65, 0x61, 0x6D, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
