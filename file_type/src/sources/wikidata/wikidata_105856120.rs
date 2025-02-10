use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856120: FileFormat = FileFormat {
    id: 105_856_120,
    source_type: SourceType::Wikidata,
    name: "Distribution Format Exchange Profile",
    extensions: &["dfxp"],
    media_types: &["application/ttml+xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
