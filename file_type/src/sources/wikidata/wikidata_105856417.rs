use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856417: FileFormat = FileFormat {
    id: 105_856_417,
    source_type: SourceType::Wikidata,
    name: "WavePad Audio Project",
    extensions: &["wpp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6C, 0x73, 0x64, 0x66, 0x01, 0x00, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
