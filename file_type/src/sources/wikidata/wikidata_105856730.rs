use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856730: FileFormat = FileFormat {
    id: 105_856_730,
    source_type: SourceType::Wikidata,
    name: "Canon printer User Mode backup",
    extensions: &["umd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x53, 0x45, 0x52, 0x4D, 0x4F, 0x44, 0x45, 0x42, 0x41, 0x43, 0x4B, 0x55,
                    0x50, 0x3D, 0x56, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
