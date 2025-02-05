use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000681: FileFormat = FileFormat {
    id: 29_000_681,
    source_type: SourceType::Wikidata,
    name: "quick3D object file",
    extensions: &["q3o"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x71, 0x75, 0x69, 0x63, 0x6B, 0x33, 0x44, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
