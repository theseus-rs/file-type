use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856225: FileFormat = FileFormat {
    id: 105_856_225,
    source_type: SourceType::Wikidata,
    name: "Summation Document Image Information Load File",
    extensions: &["dii"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B])],
            },
        }],
    }],
    related_formats: &[],
};
