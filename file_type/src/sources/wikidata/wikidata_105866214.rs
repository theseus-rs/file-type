use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866214: FileFormat = FileFormat {
    id: 105_866_214,
    puid: "wikidata/105866214",
    name: "PageSetter III document",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x17, 0x18, 0x19, 0x12, 0x50, 0x61, 0x67, 0x65, 0x53, 0x65, 0x74, 0x74, 0x65,
                    0x72, 0xB3, 0x20, 0xA9, 0x31, 0x39, 0x39, 0x32, 0x20, 0x47, 0x6F, 0x6C, 0x64,
                    0x20, 0x44, 0x69, 0x73, 0x6B, 0x20, 0x49, 0x6E, 0x63, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
