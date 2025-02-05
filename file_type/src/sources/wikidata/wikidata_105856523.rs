use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856523: FileFormat = FileFormat {
    id: 105_856_523,
    source_type: SourceType::Wikidata,
    name: "NextSTART Theme",
    extensions: &["wst"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4E, 0x45, 0x58, 0x54, 0x53, 0x54, 0x41, 0x52, 0x54, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
