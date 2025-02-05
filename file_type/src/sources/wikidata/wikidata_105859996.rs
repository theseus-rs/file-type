use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859996: FileFormat = FileFormat {
    id: 105_859_996,
    source_type: SourceType::Wikidata,
    name: "Division dVS geometry",
    extensions: &["viz"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2A, 0x20, 0x76, 0x69, 0x7A, 0x20, 0x76, 0x31, 0x2E, 0x30, 0x20, 0x28,
                    0x63, 0x29, 0x20, 0x44, 0x49, 0x56, 0x49, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x4C,
                    0x74, 0x64, 0x20, 0x31, 0x39, 0x39, 0x32, 0x20, 0x2A, 0x2F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
