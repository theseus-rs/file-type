use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858554: FileFormat = FileFormat {
    id: 105_858_554,
    source_type: SourceType::Wikidata,
    name: "ExamView Question Bank",
    extensions: &["bnk"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1A, 0x46, 0x53, 0x43, 0x42, 0x4E, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
