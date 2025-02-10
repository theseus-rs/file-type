use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855429: FileFormat = FileFormat {
    id: 105_855_429,
    source_type: SourceType::Wikidata,
    name: "Gherkin Feature",
    extensions: &["feature"],
    media_types: &["text/plain", "text/x-gherkin"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
