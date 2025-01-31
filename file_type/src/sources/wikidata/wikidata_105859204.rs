use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859204: FileFormat = FileFormat {
    id: 105_859_204,
    puid: "wikidata/105859204",
    name: "Drazpaint (C64) bitmap",
    extensions: &["drz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
