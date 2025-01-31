use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855896: FileFormat = FileFormat {
    id: 105_855_896,
    puid: "wikidata/105855896",
    name: "Dr. Solomon's Antivirus messages",
    extensions: &["drv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    0x20, 0x28, 0x63, 0x29, 0x20, 0x53, 0x26, 0x53, 0x20, 0x49, 0x6E, 0x74, 0x65,
                    0x72, 0x6E, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x61, 0x6C, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
