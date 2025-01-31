use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866751: FileFormat = FileFormat {
    id: 105_866_751,
    puid: "wikidata/105866751",
    name: "PlayStation 3 Theme",
    extensions: &["p3t"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x33, 0x54, 0x46, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x40, 0x00,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
