use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858436: FileFormat = FileFormat {
    id: 105_858_436,
    puid: "wikidata/105858436",
    name: "Keyhole - Google Earth Overlay",
    extensions: &["eta"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4B, 0x65, 0x79, 0x68, 0x6F, 0x6C, 0x65, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
