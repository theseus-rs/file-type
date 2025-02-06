use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854946: FileFormat = FileFormat {
    id: 105_854_946,
    source_type: SourceType::Wikidata,
    name: "CrossePAC compressed archive",
    extensions: &["pac"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x49, 0x47, 0x44, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
