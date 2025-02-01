use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860379: FileFormat = FileFormat {
    id: 105_860_379,
    puid: "wikidata/105860379",
    name: "RealArcade Game Package",
    extensions: &["rgp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x50, 0x41, 0x43, 0x4B, 0x41, 0x47, 0x45, 0x3E, 0x0D, 0x0A, 0x20, 0x20,
                    0x3C, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
