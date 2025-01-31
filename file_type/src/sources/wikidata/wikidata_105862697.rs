use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862697: FileFormat = FileFormat {
    id: 105_862_697,
    puid: "wikidata/105862697",
    name: "Maxwell Render Scene",
    extensions: &["mxs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF9, 0xB2, 0x2A, 0xD1])],
            },
        }],
    }],
    related_formats: &[],
};
