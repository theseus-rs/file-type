use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860351: FileFormat = FileFormat {
    id: 105_860_351,
    source_type: SourceType::Wikidata,
    name: "Richard's Bridge Notation (inline)",
    extensions: &["rbx"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x7B, 0x52, 0x42, 0x58, 0x7D])],
            },
        }],
    }],
    related_formats: &[],
};
