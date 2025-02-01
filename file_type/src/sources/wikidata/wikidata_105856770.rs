use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856770: FileFormat = FileFormat {
    id: 105_856_770,
    puid: "wikidata/105856770",
    name: "Universal Communications Format",
    extensions: &["ucf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x50, 0x44, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
