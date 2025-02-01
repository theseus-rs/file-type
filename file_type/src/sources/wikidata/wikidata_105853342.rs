use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853342: FileFormat = FileFormat {
    id: 105_853_342,
    puid: "wikidata/105853342",
    name: "Movie Magic Screenwriter document",
    extensions: &["scw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x63, 0x72, 0x65, 0x65, 0x6E, 0x77, 0x72, 0x69, 0x74, 0x65, 0x72, 0x57,
                    0x69, 0x6E, 0x56, 0x65, 0x72, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
