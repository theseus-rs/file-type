use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856797: FileFormat = FileFormat {
    id: 105_856_797,
    source_type: SourceType::Wikidata,
    name: "Arts and Letters Graphics (old)",
    extensions: &["ged"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x72, 0x74, 0x73, 0x20, 0x26, 0x20, 0x4C, 0x65, 0x74, 0x74, 0x65, 0x72,
                    0x73, 0x20, 0x45, 0x64, 0x69, 0x74, 0x6F, 0x72, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
