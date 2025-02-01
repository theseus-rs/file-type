use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860358: FileFormat = FileFormat {
    id: 105_860_358,
    puid: "wikidata/105860358",
    name: "Deluxe Ski Jump 2 Replay",
    extensions: &["rpl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x14, 0xB8, 0xA9, 0x19, 0x89, 0xFA, 0x7A, 0xED, 0x86, 0xB8, 0x37, 0x3C, 0x75,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
