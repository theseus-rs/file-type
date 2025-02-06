use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858172: FileFormat = FileFormat {
    id: 105_858_172,
    source_type: SourceType::Wikidata,
    name: "EnCase Package",
    extensions: &["enpack"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x6E, 0x46, 0x09, 0x0D, 0x0A, 0xFF, 0x00, 0x50, 0x41, 0x43, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
