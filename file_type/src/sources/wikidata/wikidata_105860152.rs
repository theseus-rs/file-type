use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860152: FileFormat = FileFormat {
    id: 105_860_152,
    puid: "wikidata/105860152",
    name: "RPG Maker VX Ace Project",
    extensions: &["rvproj2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x50, 0x47, 0x56, 0x58, 0x41, 0x63, 0x65, 0x20, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
