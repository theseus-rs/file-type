use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858046: FileFormat = FileFormat {
    id: 105_858_046,
    source_type: SourceType::Wikidata,
    name: "R.D.S. warrior load format (v0.8)",
    extensions: &["int"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x2E, 0x44, 0x2E, 0x53, 0x2E, 0x30, 0x2E, 0x38, 0x20, 0x20, 0x20, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
