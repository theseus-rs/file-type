use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960132: FileFormat = FileFormat {
    id: 27_960_132,
    puid: "wikidata/27960132",
    name: "Bonk",
    extensions: &["bonk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x42, 0x4F, 0x4E, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
