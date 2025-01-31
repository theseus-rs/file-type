use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854493: FileFormat = FileFormat {
    id: 105_854_493,
    puid: "wikidata/105854493",
    name: "Pawn compiled program",
    extensions: &["amx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0xE0, 0xF1, 0x0B, 0x0B, 0x00, 0x00, 0x08, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
