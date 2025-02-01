use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856486: FileFormat = FileFormat {
    id: 105_856_486,
    puid: "wikidata/105856486",
    name: "Wax Project",
    extensions: &["wxp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x20, 0x57, 0x61, 0x78, 0x20, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74,
                    0x65, 0x64, 0x20, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x2E, 0x20, 0x44,
                    0x6F, 0x20, 0x6E, 0x6F, 0x74, 0x20, 0x65, 0x64, 0x69, 0x74, 0x20, 0x6D, 0x61,
                    0x6E, 0x75, 0x61, 0x6C, 0x6C, 0x79, 0x21, 0x21, 0x20, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
