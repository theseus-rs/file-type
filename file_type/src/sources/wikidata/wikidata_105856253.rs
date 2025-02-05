use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856253: FileFormat = FileFormat {
    id: 105_856_253,
    source_type: SourceType::Wikidata,
    name: "Dune Service File",
    extensions: &["dsf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x55, 0x4E, 0x45, 0x20, 0x53, 0x45, 0x52, 0x56, 0x49, 0x43, 0x45, 0x20,
                    0x46, 0x49, 0x4C, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
