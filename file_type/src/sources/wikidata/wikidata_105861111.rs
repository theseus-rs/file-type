use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861111: FileFormat = FileFormat {
    id: 105_861_111,
    source_type: SourceType::Wikidata,
    name: "AIBB results log",
    extensions: &["log"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x49, 0x42, 0x42, 0x4C, 0x6F, 0x67, 0x46, 0x69, 0x6C, 0x65, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
