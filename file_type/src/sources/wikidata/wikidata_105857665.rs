use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857665: FileFormat = FileFormat {
    id: 105_857_665,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine compressed archive (v1.0)",
    extensions: &["bif"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x49, 0x46, 0x43, 0x56, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
