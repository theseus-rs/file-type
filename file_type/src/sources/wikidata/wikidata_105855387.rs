use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855387: FileFormat = FileFormat {
    id: 105_855_387,
    puid: "wikidata/105855387",
    name: "Guild Wars gamedata",
    extensions: &["ffna"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x66, 0x6E, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
