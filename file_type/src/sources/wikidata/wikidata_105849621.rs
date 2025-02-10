use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849621: FileFormat = FileFormat {
    id: 105_849_621,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM Maverick's C0DER protected",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE9])],
            },
        }],
    }],
    related_formats: &[],
};
