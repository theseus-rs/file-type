use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_13422998: FileFormat = FileFormat {
    id: 13_422_998,
    source_type: SourceType::Wikidata,
    name: "HTTP Archive format",
    extensions: &["har"],
    media_types: &["text/json"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
