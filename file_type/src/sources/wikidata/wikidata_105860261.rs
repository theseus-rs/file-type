use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860261: FileFormat = FileFormat {
    id: 105_860_261,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD 3D model info (v3.0)",
    extensions: &["rsd"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x52, 0x53, 0x44, 0x39, 0x37, 0x30, 0x34, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
