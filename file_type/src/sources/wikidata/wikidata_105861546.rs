use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861546: FileFormat = FileFormat {
    id: 105_861_546,
    source_type: SourceType::Wikidata,
    name: "Chess Assistant Library",
    extensions: &["lib"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x68, 0x65, 0x73, 0x73, 0x20, 0x41, 0x73, 0x73, 0x69, 0x73, 0x74, 0x61,
                    0x6E, 0x74, 0x20, 0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x20, 0x46, 0x69,
                    0x6C, 0x65, 0x2E, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
