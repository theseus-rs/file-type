use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852983: FileFormat = FileFormat {
    id: 105_852_983,
    source_type: SourceType::Wikidata,
    name: "Sonnet Project",
    extensions: &["son"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x54, 0x59, 0x50, 0x20, 0x53, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
