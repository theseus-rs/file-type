use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852609: FileFormat = FileFormat {
    id: 105_852_609,
    puid: "wikidata/105852609",
    name: "SWiSH project",
    extensions: &["swi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x05, 0xFF, 0xFE, 0xFF, 0x13, 0x53, 0x00, 0x57, 0x00, 0x69, 0x00, 0x53,
                    0x00, 0x48, 0x00, 0x6D, 0x00, 0x61, 0x00, 0x78, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
