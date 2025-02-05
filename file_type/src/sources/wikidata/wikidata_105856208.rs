use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856208: FileFormat = FileFormat {
    id: 105_856_208,
    source_type: SourceType::Wikidata,
    name: "SVN dump format (v1)",
    extensions: &["dump"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x56, 0x4E, 0x2D, 0x66, 0x73, 0x2D, 0x64, 0x75, 0x6D, 0x70, 0x2D, 0x66,
                    0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2D, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3A, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
