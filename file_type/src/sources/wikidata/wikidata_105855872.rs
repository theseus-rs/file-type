use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855872: FileFormat = FileFormat {
    id: 105_855_872,
    puid: "wikidata/105855872",
    name: "Code::Blocks Dependencies",
    extensions: &["depend"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x64, 0x65, 0x70, 0x73, 0x6C, 0x69, 0x62, 0x20, 0x64, 0x65, 0x70,
                    0x65, 0x6E, 0x64, 0x65, 0x6E, 0x63, 0x79, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                    0x76, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
