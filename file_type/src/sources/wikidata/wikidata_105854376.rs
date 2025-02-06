use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854376: FileFormat = FileFormat {
    id: 105_854_376,
    source_type: SourceType::Wikidata,
    name: "Atari800Win Plus Snapshot (un-gzipped)",
    extensions: &["a8s"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x54, 0x41, 0x52, 0x49, 0x38, 0x30, 0x30, 0x06,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
