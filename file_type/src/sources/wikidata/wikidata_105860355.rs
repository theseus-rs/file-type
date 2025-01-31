use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860355: FileFormat = FileFormat {
    id: 105_860_355,
    puid: "wikidata/105860355",
    name: "Farallon Replica document",
    extensions: &["rpl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x61, 0x72, 0x61, 0x6C, 0x6C, 0x6F, 0x6E, 0x20, 0x52, 0x65, 0x70, 0x6C,
                    0x69, 0x63, 0x61, 0x20, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x20, 0x20, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
