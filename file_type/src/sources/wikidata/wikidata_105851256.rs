use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851256: FileFormat = FileFormat {
    id: 105_851_256,
    source_type: SourceType::Wikidata,
    name: "Traces scene (old)",
    extensions: &["trace"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x52, 0x41, 0x36, 0x57, 0x52, 0x4C, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
