use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856139: FileFormat = FileFormat {
    id: 105_856_139,
    source_type: SourceType::Wikidata,
    name: "NextDAW project",
    extensions: &["daw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x65, 0x78, 0x74, 0x44, 0x41, 0x57, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
