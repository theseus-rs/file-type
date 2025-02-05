use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856106: FileFormat = FileFormat {
    id: 105_856_106,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing Exchange Binary Format (v1.0)",
    extensions: &["dxb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x44, 0x58, 0x42, 0x20, 0x31,
                    0x2E, 0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
