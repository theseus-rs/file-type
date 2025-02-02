use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856088: FileFormat = FileFormat {
    id: 105_856_088,
    source_type: SourceType::Wikidata,
    name: "Dante firmware update",
    extensions: &["dnt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x55, 0x44, 0x49, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
