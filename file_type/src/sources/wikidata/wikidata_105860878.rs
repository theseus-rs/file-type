use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860878: FileFormat = FileFormat {
    id: 105_860_878,
    source_type: SourceType::Wikidata,
    name: "RAD Studio Active X RIDL data",
    extensions: &["ridl"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2F, 0x20, 0x2A, 0x2A, 0x2A, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
