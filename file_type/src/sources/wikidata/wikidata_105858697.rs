use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858697: FileFormat = FileFormat {
    id: 105_858_697,
    puid: "wikidata/105858697",
    name: "Cryo Interactive gamedata",
    extensions: &["bf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x72, 0x79, 0x6F, 0x42, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
