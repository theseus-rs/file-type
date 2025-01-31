use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858411: FileFormat = FileFormat {
    id: 105_858_411,
    puid: "wikidata/105858411",
    name: "Extended CPCEMU style disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x58, 0x54, 0x45, 0x4E, 0x44, 0x45, 0x44, 0x20, 0x43, 0x50, 0x43, 0x20,
                    0x44, 0x53, 0x4B, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A, 0x44, 0x69, 0x73,
                    0x6B, 0x2D, 0x49, 0x6E, 0x66, 0x6F, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
