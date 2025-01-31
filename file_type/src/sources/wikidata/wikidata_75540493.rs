use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75540493: FileFormat = FileFormat {
    id: 75_540_493,
    puid: "wikidata/75540493",
    name: "Ulead COOL 3D",
    extensions: &["uez"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0xD8, 0x3A, 0xF7, 0x01, 0x20, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
