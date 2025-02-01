use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856141: FileFormat = FileFormat {
    id: 105_856_141,
    puid: "wikidata/105856141",
    name: "Turbo Pascal Desktop",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x75, 0x72, 0x62, 0x6F, 0x20, 0x50, 0x61, 0x73, 0x63, 0x61, 0x6C, 0x20,
                    0x44, 0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
