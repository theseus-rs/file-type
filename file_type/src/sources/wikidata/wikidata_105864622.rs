use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864622: FileFormat = FileFormat {
    id: 105_864_622,
    source_type: SourceType::Wikidata,
    name: "Protege classes",
    extensions: &["pont"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
