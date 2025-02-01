use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855769: FileFormat = FileFormat {
    id: 105_855_769,
    puid: "wikidata/105855769",
    name: "DoubleDisk compressed volume (v2.5)",
    extensions: &["000"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEB, 0x6E, 0x90, 0x49, 0x42, 0x4D, 0x4C, 0x32, 0x2E, 0x35, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
