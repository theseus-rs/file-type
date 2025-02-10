use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859093: FileFormat = FileFormat {
    id: 105_859_093,
    source_type: SourceType::Wikidata,
    name: "VITec image format bitmap",
    extensions: &["vit", "vitec"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x5B, 0x07, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
