use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5323042: FileFormat = FileFormat {
    id: 5_323_042,
    puid: "wikidata/5323042",
    name: "EGG",
    extensions: &["egg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x47, 0x47, 0x41, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
